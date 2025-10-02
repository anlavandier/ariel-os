#![no_main]
#![no_std]

use ariel_os::debug::{exit, log::{defmt, info}, ExitCode};
use ariel_os::time::{Duration, Instant, Timer, with_timeout};

use wasmtime::{Config, Engine, Store};
use wasmtime::component::{bindgen, Component, HasSelf, Linker};

// extern crate alloc;

use ariel_os_bindings::wasm::ArielHost;

bindgen!({
    world: "example",
    path: "wit/",
    with: {
        "ariel:wasm-bindings/log-api": ariel_os_bindings::wasm::log,
        "ariel:wasm-bindings/rng-api": ariel_os_bindings::wasm::rng,
        "ariel:wasm-bindings/time-api": ariel_os_bindings::wasm::time,

    },
    imports: { default: async },
    exports: {default : async }
});

#[ariel_os::task(autostart)]
async fn main() {
    let now = Instant::now();
    info!("Trying a timeout of 9 seconds, this should error");
    let r = with_timeout(Duration::from_secs(9), run_wasm()).await;
    let new_now = Instant::now();
    info!("{:?}", defmt::Debug2Format(&r));
    info!("This took {:?} ms", (new_now - now).as_millis());
    Timer::after_millis(100).await;
    exit(ExitCode::SUCCESS);
}


/// # Errors
/// Misconfiguration of Wasmtime or of the component
async fn run_wasm() -> wasmtime::Result<()> {
    let mut config = Config::default();
    config.max_wasm_stack(2048);
    config.wasm_custom_page_sizes(true);
    config.target("pulley32")?;
    config.memory_reservation(0);
    config.memory_init_cow(false);
    config.memory_reservation_for_growth(0);
    config.memory_may_move(false);
    config.consume_fuel(true);
    // Async support

    config.async_support(true);
    config.async_stack_size(4096);

    let engine = Engine::new(&config)?;
    let component_bytes = include_bytes!("../bindings-async.cwasm");

    let component = unsafe { Component::deserialize_raw(&engine, component_bytes.as_slice().into()) }?;

    let host = ArielHost::default();

    let mut store = Store::new(&engine, host);

    // Enough fuel to never run out before the timeout
    store.set_fuel(1_000_000_000)?;

    // Yield every 10_000 fuel expanded to allow for timeouts
    store.fuel_async_yield_interval(Some(10_000))?;

    let mut linker = Linker::new(&engine);

    Example::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| {state})?;
    let bindings = Example::instantiate_async(&mut store, &component, &linker).await?;

    bindings.call_do_work(&mut store).await?;

    Ok(())
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