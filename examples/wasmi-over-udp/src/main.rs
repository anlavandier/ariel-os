#![no_main]
#![no_std]

use ariel_os::{debug::{exit, log::{defmt, *}, ExitCode}, net, reexports::embassy_net};
use embassy_net::udp::{PacketMetadata, UdpSocket};

// use core::slice;

use wasmi::{Engine, Module, Linker, Store, Caller};

const BUFFER_SIZE: usize = 1024;
const WASM_BUFFER_SIZE: usize = 1024;

#[ariel_os::task(autostart)]
async fn main() {
    info!("Testing Wasmi on this board by sending files over udp");


    let stack = net::network_stack().await.unwrap();

    let mut rx_meta = [PacketMetadata::EMPTY; 1];
    let mut rx_buffer = [0; BUFFER_SIZE];
    let mut tx_meta = [PacketMetadata::EMPTY; 1];
    let mut tx_buffer = [0; BUFFER_SIZE];
    // let mut buf = [0; BUFFER_SIZE];

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

    info!("Ready to receive WASM Payload of up to {} bytes", WASM_BUFFER_SIZE);
    let mut wasm_buffer = [0; WASM_BUFFER_SIZE];
    let file_size = match socket.recv_from(&mut wasm_buffer).await {
        Ok((0, _)) => {
            0
        },
        Ok((n, _)) => {
            n
        }
        Err(_e) => {
            0
        }
    };

    let wasm_r = run_wasm(&wasm_buffer.split_at(file_size).0);
    match wasm_r {
        Ok(v)  => {
            info!("Got {} by running wasm", v);
        },
        Err(e) => {
            info!("{}", defmt::Display2Format(&e));
        }
    }
}

fn run_wasm(wasm: &[u8]) -> Result<u32, wasmi::Error> {

    // let wasm  = include_bytes!("../low_mem_example.wasm");
    // the nrf52840 chip has 1MB of flash, we put the wasm at the adress 0xABE60
    // probe-rs download low_mem_example.wasm --binary-format bin --chip nrf52840_xxAA --base-address 0xABE60
    // let wasm = unsafe { slice::from_raw_parts(0xABE60 as *const u8, 146)};
    // First step is to create the Wasm execution engine with some config.
    // In this example we are using the default configuration.
    let engine = Engine::default();
    // Now we can compile the above Wasm module with the given Wasm source.
    let module = Module::new(&engine, wasm)?;


    //
    for imp in module.imports() {
        info!("{:?}", defmt::Debug2Format(&imp));
    }
    for exp in module.exports() {
        info!("{:?}", defmt::Debug2Format(&exp));
    }

    // Wasm objects operate within the context of a Wasm `Store`.
    //
    // Each `Store` has a type parameter to store host specific data.
    // In this example the host state is a simple `u32` type with value `42`.
    type HostState = u32;
    let mut store = Store::new(&engine, 42);

    info!("Created Store");
    // A linker can be used to instantiate Wasm modules.
    // The job of a linker is to satisfy the Wasm module's imports.

    let mut linker = <Linker<HostState>>::new(&engine);
    info!("Created Linker");
    // We are required to define all imports before instantiating a Wasm module.
    let _ = linker.func_wrap("host", "host_hello", |caller: Caller<'_, HostState>, param: u32| {
        info!("Got {} from WebAssembly and my host state is: {}",param, caller.data());
    })?;

    info!("Linking Done");
    let instance = linker.instantiate(&mut store, &module)?
        .start(&mut store)?;

    info!("Instantiating done");
    // Now we can finally query the exported "hello" function and call it.
    let res = instance
        .get_typed_func::<(u32, u32), u32>(&store, "add")?
        .call(&mut store, (2, 2))?;
    Ok(res)
    // Ok(0)
}

