#![no_main]
#![no_std]

#![allow(unused_imports)]


#[cfg(not(target_pointer_width = "32"))]
compile_error!("Only works on 32 bit architecture");

use ::core::{str, slice};
use ariel_os::{debug::{exit, log::{defmt, info}, ExitCode}, random::fast_rng, time::Timer};
// use core::slice;
use wasmi::{AsContext, AsContextMut, Caller, Config, Engine, Extern, Func, Instance, Linker, Memory, MemoryType, Module, Store};
use ariel_os_bindings::wasm::log::export_log;

use core::cell::RefCell;

use rand::RngCore;

use zerocopy::{TryFromBytes, IntoBytes, KnownLayout, Immutable};

const PACKET_LEN: usize = 128;

#[repr(C)]
#[derive(Default, defmt::Format, Debug, TryFromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct UDPPacketHeader {
    source: u16,
    dest: u16,
    length: u16,
    checksum: u16,
}

#[repr(C)]
#[derive(defmt::Format, Debug, TryFromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct UDPPacket {
    header:UDPPacketHeader,
    data: [u8; PACKET_LEN],
}

#[ariel_os::task(autostart)]
async fn main() {
    info!("Testing Wasmi on this board");
    let wasm_r = run_wasm();
    match wasm_r {
        Ok(_v)  => {
            // info!("Got {} from wasm", _v);
        },
        Err(e) => {
            info!("{}", defmt::Display2Format(&e));
        }
    }
    Timer::after_millis(100).await;
    exit(ExitCode::SUCCESS);
}

/// # Errors
/// - When Wasmi isn't happy
/// # Panics
/// - When Wasm gives us a non-null but wrong pointer
fn run_wasm() -> Result<i32, wasmi::Error> {
    // info!("Trying to run wasm");

    let wasm  = include_bytes!("../payload.wasm");

    // First step is to create the Wasm execution engine with some config.
    let mut config = Config::default();

    // Enable the custom page sizes proposal to use less RAM
    config.wasm_custom_page_sizes(true);
    let engine = Engine::new(&config);

    // Now we can compile the above Wasm module with the given Wasm source.
    let module = Module::new(&engine, wasm)?;

    // Wasm Module run in a Store that contains some data
    info!("Create Store");
    let mut store: Store<u32> = Store::new(&engine, 42);

    // // Initiate linker for easier definition of host functions
    let mut linker = Linker::new(&engine);

    linker.func_wrap("host", "do_stuff_with_packet", |caller: Caller<'_, u32>, packet_ptr: u32| {
        if let Some(Extern::Memory(mem)) = caller.get_export("memory") {
            let udp_packet_size = core::mem::size_of::<UDPPacket>();
            let udp_ptr = packet_ptr as usize;
            if let Some(udp_slice) = mem.data(caller.as_context()).get(udp_ptr..udp_ptr + udp_packet_size) {
                let mypacket: &UDPPacket = UDPPacket::try_ref_from_bytes(udp_slice).unwrap();
                info!("Got this packet: {:?}", mypacket);
            }
        }
    })?;
    // // Using the premade bindings
    // export_log(&mut linker)?;


    linker.func_wrap("rng", "__next_u32", || {
        let mut frng=  fast_rng();
        frng.next_u32()
    })?;
    linker.func_wrap("rng", "__next_u64", || {
        let mut frng=  fast_rng();
        frng.next_u64()
    })?;

    linker.func_wrap("rng", "__fill_bytes", |mut caller: Caller<'_, u32>, ptr: u32, len: u32| {
        if let Some(Extern::Memory(mem)) = caller.get_export("memory") {
            let slice_ptr = ptr as usize;
            let slice_len = len as usize;
            if let Some(b_slice) = mem.data_mut(caller.as_context_mut()).get_mut(slice_ptr..slice_ptr + slice_len) {
                let mut frng=  fast_rng();
                frng.fill_bytes(b_slice);
            }
        }
    })?;


    linker.func_wrap("host", "log_u32", |i: u32| {
        info!("Got {}", i);
    })?;
    linker.func_wrap("host", "log_u64", |i: u64| {
        info!("Got {}", i);
    })?;

    linker.func_wrap("host", "log_bytes", |caller: Caller<'_, u32>, ptr: u32, len: u32| {
        if let Some(Extern::Memory(mem)) = caller.get_export("memory") {
            let slice_ptr = ptr as usize;
            let slice_len = len as usize;
            if let Some(b_slice) = mem.data(caller.as_context()).get(slice_ptr..slice_ptr + slice_len) {
                info!("Got {:?}", b_slice);
            }
        }
    })?;

    // Initiate Instance with the import function
    let instance = linker.instantiate(&mut store, &module)?.start(&mut  store)?;
    info!("Finalized instance");

    // Use ZeroCopy on client side to serialize a struct
    let test_func = instance.get_typed_func::<(), ()>(&store, "hello")?;
    test_func.call(&mut store, ())?;

    // Use Zerocopy on client side to deserialize a struct initialized on host side.
    let (size, align) = (size_of::<UDPPacket>(), align_of::<UDPPacket>());
    let packet_allocated_addr = instance.get_typed_func::<(u32, u32), u32>(&mut store, "alloc")?
        .call(&mut store, (size as u32, align as u32))?;

    let default_packet  = UDPPacket {
        header: UDPPacketHeader::default(),
        data: [0; PACKET_LEN]
    };
    if let Some(mem) = instance.get_memory(&mut store, "memory") {
        mem.write(&mut store, packet_allocated_addr as usize, default_packet.as_bytes())?;
    }
    instance.get_typed_func::<u32, ()>(&mut store, "mutate_packet")?.call(&mut store, packet_allocated_addr)?;

    let rng_test_func = instance.get_typed_func::<(), ()>(&mut store, "rng_test")?;

    for _ in 0..5 {
        rng_test_func.call(&mut store, ())?;
    }

    Ok(0)
}