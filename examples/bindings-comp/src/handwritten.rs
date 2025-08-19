pub use ariel_os_bindings::wasm::rng::hand_written::host_api::export_rng;

use wasmtime::{Config, Engine, Module, Linker, Store};

use super::info;

///
/// # Panics
///
/// # Errors
///
pub fn run_wasm() -> wasmtime::Result<u64> {
    info!("Using the handwritten bindings");
    // Configuration used when precompiling the module
    let mut config = Config::default();
    config.max_wasm_stack(2048);
    config.wasm_custom_page_sizes(true);
    config.target("pulley32")?;
    config.memory_reservation(0);
    config.memory_init_cow(false);
    config.memory_reservation_for_growth(0);
    config.memory_may_move(false);
    let engine = Engine::new(&config)?;

    let module_bytes = include_bytes!("../payload_handwritten.cwasm");

    let module = unsafe { Module::deserialize_raw(&engine, module_bytes.as_slice().into())? };

    let mut store = Store::new(&engine, ());
    let mut linker = Linker::new(&engine);

    export_rng(&mut linker)?;

    let instance = linker.instantiate(&mut store, &module)?;
    let sum = instance.get_typed_func::<(), u64>(&mut store, "start")?
        .call(&mut store, ())?;
    Ok(sum)
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
