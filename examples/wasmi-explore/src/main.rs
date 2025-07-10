#![no_main]
#![no_std]

#![allow(unused_imports)]


use ::core::{str, slice};

use ariel_os::{debug::{exit, log::{defmt, info}, ExitCode}};
// use core::slice;
use wasmi::{Config, Linker, Engine, Store, Module};
use ariel_os_bindings::wasm::log::log_str_builder;




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
    exit(ExitCode::SUCCESS);
}

///
/// # Errors
/// - When Wasmi isn't happy
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



    // Initiate linker for easier definition of host functions
    let mut linker = Linker::new(&engine);

    // Define a host function through a closure
    // Host Functions can only have args/return types that implement
    // the wasmi::WasmTy trait. In particular, for integers, only u/i 32/64
    // can be used.

    // Using the log_str_builder
    linker.func_wrap("log", "log_str", log_str_builder())?;

    // Initiate Instance with the import function
    let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;
    info!("Finalized instance");

    // Allocates a string, and print it using the host capabilities.
    let wasm_logging_static = instance.get_typed_func::<(), ()>(&store, "static_alloc_and_log")?;
    wasm_logging_static.call(&mut store, ())?;

    Ok(0)
}