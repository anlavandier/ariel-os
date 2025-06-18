#![no_main]
#![no_std]

use ariel_os::{debug::{exit, log::{defmt, *}, ExitCode}, net, reexports::embassy_net};
use embassy_net::udp::{PacketMetadata, UdpSocket};
use wasmi::{Engine, Module, Linker, Store};

const BUFFER_SIZE: usize = 128;
const WASM_BUFFER_SIZE: usize = 1024;
const USIZE_BYTES: usize = (usize::BITS / 8) as usize;


#[ariel_os::task(autostart)]
async fn main() {
    info!("Testing Wasmi on this board by sending files over udp");


    let stack = net::network_stack().await.unwrap();

    let mut rx_meta = [PacketMetadata::EMPTY; 1];
    let mut rx_buffer = [0; BUFFER_SIZE];
    let mut tx_meta = [PacketMetadata::EMPTY; 1];
    let mut tx_buffer = [0; BUFFER_SIZE];
    let mut buf = [0; BUFFER_SIZE];

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
    let file_size = match socket.recv_from(&mut buf).await {
        Ok((0, _)) => {
            0
        },
        Ok((n, _)) => {
            assert_eq!(n, USIZE_BYTES);
            usize::from_be_bytes(buf[..USIZE_BYTES].try_into().expect("Bug"))
        }
        Err(_e) => {
            0
        }
    };

    info!("Received {} as wasm file size", file_size);
    if file_size > WASM_BUFFER_SIZE {
        info!("Wasm payload is too big for the allocated buffer");
        exit(ExitCode::FAILURE);
    }
    let mut offset = 0;
    while offset != file_size {
        match socket.recv_from(&mut buf).await {
            Ok((0, _)) => {
                break;
            }
            Ok((n, _)) => {
                info!("Received {} bytes of the file", n);
                wasm_buffer[offset..offset+n].copy_from_slice(&buf[..n]);
                offset+=n;

            },
            _ => {
                break;
            }
        }
    }

    if offset != file_size {
        info!("Wasm Payload wasn't properly sent");
        exit(ExitCode::FAILURE);
    }

    info!("Launching Wasm");

    let wasm_r = run_wasm_main(&wasm_buffer[..file_size]);
    match wasm_r {
        Err(e) => {
            info!("{}", defmt::Display2Format(&e));
        },
        Ok(r) => { info!("Got {} from wasm", r)},
    }
    exit(ExitCode::SUCCESS);
}



/// Function that loads a wasm payload and runs its "main" function
fn run_wasm_main(wasm: &[u8]) -> Result<u32, wasmi::Error> {

    // First step is to create the Wasm execution engine with some config.
    // In this example we are using the default configuration.
    let engine = Engine::default();
    // Now we can compile the above Wasm module with the given Wasm source.
    let module = Module::new(&engine, wasm)?;

    // Wasm objects operate within the context of a Wasm `Store`.
    //
    // Each `Store` has a type parameter to store host specific data.
    // In this example the host state is a simple `u32` type with value `42`.
    type HostState = u32;
    let mut store = Store::new(&engine, 42);

    info!("Created Store");
    // A linker can be used to instantiate Wasm modules.
    // The job of a linker is to satisfy the Wasm module's imports.

    let linker = <Linker<HostState>>::new(&engine);
    info!("Created Linker");

    // let _ = linker.func_wrap::<u32, ()>("log", "info",
    //     |msg_ptr:u32| {

    //         info!("Got message stored at adress: {:?}", defmt::Debug2Format(&(msg_ptr as *const u8)));
    //         // let msg = unsafe {
    //         //     str::from_utf8(slice::from_raw_parts(msg_ptr as *const u8, msg_len as usize))
    //         // }.unwrap();
    //         // info!("{}", msg);
    //     }
    // )?;
    info!("Linking Done");

    let instance = linker.instantiate(&mut store, &module)?
        .start(&mut store)?;

    info!("Instantiating done");
    // Now we can finally query the exported "main" function and call it.
    let res = instance
        .get_typed_func::<u32, u32>(&store, "main")?
        .call(&mut store, 42)?;
    Ok(res)
}

