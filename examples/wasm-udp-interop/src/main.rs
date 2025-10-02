#![no_main]
#![no_std]

use ariel_os::{debug::log::*, time::Timer};
use ariel_os::debug::{ExitCode, exit};

use ariel_os::net::network_stack;
pub use ariel_os::net::NetworkStack as Stack;
pub use ariel_os::reexports::embassy_net::udp::{PacketMetadata, UdpSocket as Socket, UdpMetadata};
use embassy_futures::block_on;

const BUFFER_SIZE: usize = 128;

static mut RX_META: [PacketMetadata; 1] = [PacketMetadata::EMPTY; 1];
static mut RX_BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
static mut TX_META: [PacketMetadata; 1] = [PacketMetadata::EMPTY; 1];
static mut TX_BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

#[ariel_os::task(autostart)]
async fn main() {
    info!(
        "Hello from main()! Running on a {} board.",
        ariel_os::buildinfo::BOARD
    );
    if let Err(e) = run_wasmtime().await {
        info!("{:?}", defmt::Debug2Format(&e));
    }

    Timer::after_millis(100).await;
    exit(ExitCode::SUCCESS);
}


use wasmtime::{AsContext, AsContextMut, Caller, Config, Engine, Extern, Linker, Module, Store};

use wasmtime::component::{bindgen, Component, HasSelf, Linker as CLinker, Resource};

bindgen!({
    world: "example-udp-interop",
    path: "wit/udp.wit",
    // with: {
    //     "ariel:wasm-bindings/udp/udp-socket": ,
    // },
});

use ariel::wasm_bindings::udp::{Host, HostUdpSocket};


pub struct MyState{
    s: Socket<'static>,
    memed_endpoint: Option<UdpMetadata>,
}

extern crate alloc;
use alloc::vec::Vec;

impl Host for MyState {}
impl HostUdpSocket for MyState {
    fn try_recv(&mut self) -> Option<Vec<u8>> {
        let mut return_vec = Vec::from_iter(core::iter::repeat_n(0, BUFFER_SIZE));
        if self.s.may_recv() {
            match block_on(self.s.recv_from(&mut return_vec)) {
                Ok((n, ep)) => {
                    self.memed_endpoint = Some(ep);
                    return_vec.truncate(n);
                    Some(return_vec)
                },
                Err(_) => None,
            }
        }
        else {
            None
        }
    }

    fn send(&mut self, data: Vec<u8>) -> Result<(), ()> {
        let endpoint = self.memed_endpoint.unwrap();
        block_on(self.s.send_to(&data, endpoint)).map_err(|_| ())
    }

    fn bind(&mut self, port: u16) -> Result<(), ()> {

        let r = self.s.bind(port);
        info!("Started to listen on port {}, {:?}", port, r);

        r.map_err(|_| ())
    }

    fn drop(&mut self, _: Resource<UdpSocket>) -> wasmtime::Result<()> {
        // The ressource handle is never constructed and thus never dropped
        unreachable!()
    }
  }



#[allow(static_mut_refs)]
async fn run_wasmtime() -> wasmtime::Result<()> {
    let mut config = Config::default();
    config.max_wasm_stack(2048);
    config.wasm_custom_page_sizes(true);
    config.target("pulley32")?;
    config.memory_reservation(0);
    config.memory_init_cow(false);
    config.memory_reservation_for_growth(0);
    config.memory_may_move(false);
    let engine = Engine::new(&config)?;


    // let bytes = include_bytes!("../udp-interop-example.cwasm");

    // let module = unsafe { Module::deserialize_raw(&engine, bytes.as_slice().into()).unwrap() };

    let c_bytes = include_bytes!("../udp-interop-component.cwasm");
    let component = unsafe { Component::deserialize_raw(&engine, c_bytes.as_slice().into())} ?;

    let stack = network_stack().await.unwrap();

    let mut socket = unsafe { Socket::new(
        stack,
        RX_META.as_mut_slice(),
        RX_BUFFER.as_mut_slice(),
        TX_META.as_mut_slice(),
        TX_BUFFER.as_mut_slice()
    ) };
    // socket.bind(1234).unwrap();



    // let mut store = Store::new(&engine, socket);
    // let mut linker = Linker::new(&engine);

    // linker.func_wrap("udp-test", "try_recv", |mut caller: Caller<'_, Socket<'_>>, buf_ptr: u32, buf_len: u32, endpoint_ptr: u32| {
    //     let buf_ptr = buf_ptr as usize;
    //     let buf_len = buf_len as usize;
    //     let endpoint_ptr = endpoint_ptr as usize;
    //     assert!(buf_len >= BUFFER_SIZE);

    //     let mem = match caller.get_export("memory") {
    //         Some(Extern::Memory(mem)) => mem,
    //         _ => unreachable!(),
    //     };
    //     let socket = caller.data();
    //     let mut buffer = [0; BUFFER_SIZE];
    //     if socket.may_recv() {
    //         match block_on(socket.recv_from(
    //             &mut buffer
    //         )) {
    //             Ok((n, endpoint)) => {
    //                 info!("Nice ! Received datagram from {:?}", endpoint);
    //                 let port = endpoint.endpoint.port;
    //                 let addr = match endpoint.endpoint.addr.into() {
    //                     core::net::IpAddr::V4(addr) => {
    //                         addr.to_bits().to_ne_bytes()
    //                     },
    //                     _ => unreachable!("Using IPv4 only for now"),
    //                 };
    //                 assert!(n <= buf_len);
    //                 mem.data_mut(caller.as_context_mut())[buf_ptr..buf_ptr + n].copy_from_slice(&buffer[..n]);
    //                 mem.data_mut(caller.as_context_mut())[endpoint_ptr..endpoint_ptr + 4].copy_from_slice(addr.as_slice());
    //                 mem.data_mut(caller.as_context_mut())[endpoint_ptr + 4..endpoint_ptr + 6].copy_from_slice(port.to_be_bytes().as_slice());
    //                 n as u32
    //             }
    //             Err(e) => {
    //                 info!("Error: {:?}", e);
    //                 0_u32
    //             }
    //         }
    //     }
    //     else {
    //         info!("Not ready to receive datagrams");
    //         0_u32
    //     }

    // })?;

    // linker.func_wrap("udp-test", "send", |mut caller: Caller<'_, Socket<'_>>, buf_ptr: u32, buf_len: u32, endpoint_ptr: u32| {
    //     let buf_ptr = buf_ptr as usize;
    //     let buf_len = buf_len as usize;
    //     let endpoint_ptr = endpoint_ptr as usize;
    //     assert!(buf_len <= BUFFER_SIZE);


    //     let (buf, endpoint) = match caller.get_export("memory") {
    //         Some(Extern::Memory(mem)) => {
    //             let ip = core::net::Ipv4Addr::from_bits(u32::from_ne_bytes(mem.data(caller.as_context())[endpoint_ptr..endpoint_ptr + 4].try_into().unwrap()));
    //             let port = u16::from_be_bytes(mem.data(caller.as_context())[endpoint_ptr + 4..endpoint_ptr + 6].try_into().unwrap());
    //             let endpoint = core::net::SocketAddrV4::new(ip, port);
    //             (&mem.data(caller.as_context())[buf_ptr..buf_ptr + buf_len], endpoint)
    //         }
    //         _ => unreachable!()
    //     };
    //     info!("Try to send to {}", defmt::Display2Format(&endpoint));
    //     let socket = caller.data();
    //     if let Err(e) = block_on(socket.send_to(buf, endpoint)) {
    //         info!("{:?}", e);
    //     }
    // })?;

    // let instance = linker.instantiate(&mut store, &module)?;

    // let func = instance.get_typed_func::<(), ()>(&mut store, "udp_echo_plus_one")?;

    let mut comp_store = Store::new(&engine, MyState{
        s: socket,
        memed_endpoint: None,
    });
    let mut comp_linker = CLinker::new(&engine);

    ExampleUdpInterop::add_to_linker::<MyState, HasSelf<MyState>>(&mut comp_linker, |state| {state})?;

    let bindings = ExampleUdpInterop::instantiate(&mut comp_store, &component, &comp_linker)?;

    bindings.call_bind_socket(&mut comp_store).unwrap();

    loop {
        bindings.call_try_recv_then_send_back(&mut comp_store).unwrap();
        // func.call(&mut store, ())?;
        Timer::after_millis(1000).await;

    }
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
