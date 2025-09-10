#![no_main]
#![no_std]


use ariel_os::debug::{exit, log::*, ExitCode};
use ariel_os::time::Timer;
use ariel_os::{net, reexports::embassy_net};

use embassy_net::udp::{PacketMetadata, UdpSocket};
use embassy_futures::select::{select, Either};

extern crate alloc;
use alloc::{vec::Vec, sync::Arc};

use lock_api::{Mutex, RawMutex, GuardSend};

use core::sync::atomic::{AtomicBool, Ordering};

pub struct RawSpinlock(AtomicBool);

unsafe impl RawMutex for RawSpinlock {
    const INIT: RawSpinlock = RawSpinlock(AtomicBool::new(false));

    // A spinlock guard can be sent to another thread and unlocked there
    type GuardMarker = GuardSend;

    fn lock(&self) {
        // Note: This isn't the best way of implementing a spinlock, but it
        // suffices for the sake of this example.
        while !self.try_lock() {}
    }

    fn try_lock(&self) -> bool {
        self.0
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    unsafe fn unlock(&self) {
        self.0.store(false, Ordering::Release);
    }
}


use wasmtime::{AsContext, AsContextMut, Caller, Config, Engine, Extern, Linker, Module, Store, Instance};

const BUFFER_SIZE: usize = 128;



fn set_up_wasm() -> Engine {
    let mut config = Config::default();
    config.max_wasm_stack(2048);
    config.wasm_custom_page_sizes(true);
    config.target("pulley32").unwrap();
    config.memory_reservation(0);
    config.memory_init_cow(false);
    config.memory_reservation_for_growth(0);
    config.memory_may_move(false);
    let engine = Engine::new(&config).unwrap();
    engine
}


fn start_wasm(
    engine: &Engine,
    store: &mut Store<()>,
    buffer_rcv: Arc<Mutex<RawSpinlock, Vec<(usize, [u8; BUFFER_SIZE])>>>,
    buffer_send: Arc<Mutex<RawSpinlock, Vec<(usize, [u8; BUFFER_SIZE])>>>,
) -> Instance {
    let module_bytes = include_bytes!("../udp_example.cwasm");
    let module = match unsafe { Module::deserialize_raw(engine, module_bytes.as_slice().into()) } {
        Ok(module) => module,
        Err(err) => {
            info!("Error initiliazing wasm: {:?}", defmt::Debug2Format(&err));
            exit(ExitCode::FAILURE);
            loop {}
        }
    };

    let mut linker = Linker::new(engine);


    linker.func_wrap("udp-test", "blocking_receive_packet", move |mut caller: Caller<'_, ()>, res_ptr: u32| {
        // info!("Trying to find packet");
        if let Some((data_size, data)) = buffer_rcv.lock_arc().pop() {
            // Put the data in memory
            match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => {
                    let mutable_mem = mem.data_mut(caller.as_context_mut());
                    mutable_mem[res_ptr as usize..res_ptr as usize + BUFFER_SIZE].copy_from_slice(&data);
                }
                _ => unreachable!()
            }
            data_size as u32
        }
        else {
            0
        }
    }).unwrap();

    linker.func_wrap("udp-test", "blocking_send_packet", move |mut caller: Caller<'_, ()>, res_ptr: u32, data_size: u32| {
        match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => {
                let mem = mem.data(caller.as_context());
                let mut data = [0u8; BUFFER_SIZE];
                data[..data_size as usize].copy_from_slice(&mem[res_ptr as usize..res_ptr as usize + data_size as usize]);
                buffer_send.lock_arc().push((data_size as usize, data));
            }
            _ => unreachable!()
        }
    }).unwrap();
    let instance = linker.instantiate(store, &module).unwrap();
    instance
}

// Same as https://github.com/bytecodealliance/wasmtime/blob/main/examples/min-platform/embedding/wasmtime-platform.c
// I have no idea whether this is safe or not.
// https://github.com/bytecodealliance/wasmtime/blob/aec935f2e746d71934c8a131be15bbbb4392138c/crates/wasmtime/src/runtime/vm/traphandlers.rs#L888
static mut TLS_PTR: u32 = 0;
#[unsafe(no_mangle)]
extern "C" fn wasmtime_tls_get() -> *mut u8 {
    unsafe { TLS_PTR as *mut u8 }
}

#[unsafe(no_mangle)]
extern "C" fn wasmtime_tls_set(val: *const u8) {
   unsafe { TLS_PTR = val as u32 };
}

#[ariel_os::task(autostart)]
async fn main() {

    let buffer_send:Arc<Mutex<RawSpinlock, Vec<(usize, [u8; BUFFER_SIZE])>>> = Arc::new(Mutex::new(Vec::new()));
    let buffer_recv:Arc<Mutex<RawSpinlock, Vec<(usize, [u8; BUFFER_SIZE])>>> = Arc::new(Mutex::new(Vec::new()));


    let stack = net::network_stack().await.unwrap();

    let mut rx_meta = [PacketMetadata::EMPTY; 1];
    let mut rx_buffer = [0; BUFFER_SIZE];
    let mut tx_meta = [PacketMetadata::EMPTY; 1];
    let mut tx_buffer = [0; BUFFER_SIZE];
    info!("Instantiate Socket");
    let mut socket = UdpSocket::new(
        stack,
        &mut rx_meta,
        &mut rx_buffer,
        &mut tx_meta,
        &mut tx_buffer,
    );

    info!("Listening on UDP:1234...");
    if let Err(e) = socket.bind(1234) {
        info!("bind error: {:?}", e);
        exit(ExitCode::FAILURE);
    }

    let mut buf = [0; BUFFER_SIZE];
    info!("Networking initialized");

    let engine = set_up_wasm();

    let mut store = Store::new(&engine, ());
    let instance = start_wasm(&engine, &mut store, buffer_recv.clone(), buffer_send.clone());

    let start_func = instance.get_typed_func::<(), ()>(&mut store, "do_stuff").unwrap();

    let mut remote_endpoint = None;
    info!("Entering Main loop");
    loop {
        match select(socket.recv_from(&mut buf), async { Timer::after_millis(1000).await;  buffer_send.try_lock_arc().map(|mut buf| buf.pop()).flatten()}).await {
            Either::First(recv_event) => {
                match recv_event {
                    Ok((0, _)) => {
                        info!("read EOF");
                        break;
                    }
                    Ok((n, endpoint)) => {
                        remote_endpoint = Some(endpoint);
                        info!("Got Packet from {:?}", endpoint);
                        let mut data = [0u8; BUFFER_SIZE];
                        data[..n].copy_from_slice(&buf[..n]);
                        // Put the
                        buffer_recv.lock_arc().push((n, data));
                        start_func.call(&mut store, ()).unwrap();
                    }
                    Err(e) => {
                        info!("read error: {:?}", e);
                        break;
                    }
                }
            },
            Either::Second(Some((data_size, data))) => {
                info!("Found a packet to send back");
                socket.send_to(&data[..data_size], remote_endpoint.unwrap()).await.unwrap();
            }
            Either::Second(None) => {
                info!("Nothing happened");
            }
        };
    }
}