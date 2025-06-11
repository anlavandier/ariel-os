#![no_main]
#![no_std]

use ariel_os::debug::{exit, log::{defmt, *}, ExitCode};
use core::slice;

use wasmi::{Engine, Module, Linker, Store, Caller};



#[ariel_os::task(autostart)]
async fn main() {
    info!("Testing Wasmi on this board");
    let wasm_r = run_wasm();
    match wasm_r {
        Ok(v)  => {
            info!("Got {} by running wasm", v);
        },
        Err(e) => {
            info!("{}", defmt::Display2Format(&e));
        }
    }
    exit(ExitCode::SUCCESS);
}

fn run_wasm() -> Result<u32, wasmi::Error> {

    // let wasm  = include_bytes!("../low_mem_example.wasm");
    // the nrf52840 chip has 1MB of flash, we put the wasm at the adress 0xABE60
    // probe-rs download low_mem_example.wasm --binary-format bin --chip nrf52840_xxAA --base-address 0xABE60
    let wasm = unsafe { slice::from_raw_parts(0xABE60 as *const u8, 146)};
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