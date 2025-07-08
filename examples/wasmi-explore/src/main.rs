#![no_main]
#![no_std]

#![allow(unused_imports)]


use ::core::{str, slice};

use ariel_os::{debug::{exit, log::{defmt, *}, ExitCode}};
// use core::slice;
use wasmi::*;




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

    linker.func_wrap("host", "host_hello", |caller: Caller<'_, u32> | info!("Wasm Host with data: {} saying Hello!", caller.data()))?;
    linker.func_wrap("host", "log", |caller: Caller<'_, u32>, ptr_len: u64| {
        let str_ptr = (ptr_len >> 32) as usize;
        let str_len = (ptr_len & (u32::MAX as u64)) as usize;
        // Get the inner memory that was exported by the module
        match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => {
                // SAFETY: u32 and usize are the same type for our 32 bit architectures
                // Otherwise we have to trust the input parameters
                let str_slice = &mem.data(caller.as_context())[str_ptr..str_ptr + str_len];
                match str::from_utf8(str_slice) {
                    Ok(string_to_log) => { info!("WASM wanted to say this: {}", string_to_log) },
                    Err(_) => {info!("Got invalid UTF8 from WASM")}
                }
            }
            _ => unreachable!()
        }
    })?;

    // Initiate Instance with the import function
    let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;
    info!("Finalized instance");

    // call host function from wasm
    let wasm_calling_host = instance.get_typed_func::<(), ()>(&store, "hello_from_host")?;

    wasm_calling_host.call(&mut store, ())?;


    // Allocates a string, and print it using the host capabilities.
    let wasm_logging_static = instance.get_typed_func::<(), ()>(&store, "static_alloc_and_log")?;

    wasm_logging_static.call(&mut store, ())?;

    let _wasm_logging_dynamic = instance.get_typed_func::<u32, ()>(&store, "dyn_alloc_and_log");
    match _wasm_logging_dynamic {
        Ok(_f) => {
            _f.call(&mut store, 1).unwrap();
        }
        e  => {return Err(e.unwrap_err());}
    }
    // let res = wasm_logging_dynamic.call(&mut store, 5);

    // res.map(|()| 0)
    Ok(0)
}