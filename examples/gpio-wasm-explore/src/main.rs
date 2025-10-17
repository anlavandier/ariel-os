#![no_main]
#![no_std]

use ariel_os::{debug::{exit, log::{defmt, info}, ExitCode}};
use ariel_os::time::{Duration, Instant, Timer, with_timeout};
use ariel_os::gpio::{Input, Level, Output, Pull, IntEnabledInput};

use wasmtime::{AsContextMut, Caller, Config, Engine, Store};
use wasmtime::{Module, Linker, Extern};

extern crate alloc;
use alloc::boxed::Box;

mod pins;


#[ariel_os::task(autostart, peripherals)]
async fn main(peris: pins::Peripherals) {
    let now = Instant::now();



    let r = with_timeout(Duration::from_secs(20), run_wasm(peris)).await;
    let new_now = Instant::now();
    info!("{:?}", defmt::Debug2Format(&r));
    info!("This took {:?} ms", (new_now - now).as_millis());
    Timer::after_millis(100).await;
    exit(ExitCode::SUCCESS);
}


struct GPIOs {
    led: Output,
    btn: IntEnabledInput,
}

/// # Errors
/// Misconfiguration of Wasmtime or of the component
/// # Panics
/// Periph
async fn run_wasm(peris: pins::Peripherals) -> wasmtime::Result<()> {
    let mut config = Config::default();
    config.max_wasm_stack(2048);
    config.wasm_custom_page_sizes(true);
    config.target("pulley32")?;
    config.memory_reservation(0);
    config.memory_init_cow(false);
    config.memory_reservation_for_growth(0);
    config.memory_may_move(false);
    config.async_support(true);
    config.async_stack_size(4096);


    let led1 = Output::new(peris.led1, Level::Low);
    let pull = Pull::Up;

    let btn1 = Input::builder(peris.btn1, pull)
        .build_with_interrupt()
        .unwrap();


    let gpios = GPIOs { led: led1, btn: btn1};

    let engine = Engine::new(&config)?;

    let component_bytes = include_bytes!("../wasm_payload.cwasm");

    let module = unsafe { Module::deserialize_raw(&engine, component_bytes.as_slice().into()) }?;


    let mut store = Store::new(&engine, gpios);


    let mut linker = Linker::<GPIOs>::new(&engine);

    linker.func_wrap_async("test", "drive_led", |mut caller: Caller<'_, GPIOs>, ()| {
        Box::new( async move {
            caller.data_mut().led.toggle();
            info!("Timing out for 500 ms");
            Timer::after_millis(500).await;
            })
    })?;

    linker.func_wrap_async("test", "wait_for_low", |mut caller: Caller<'_, GPIOs>, () | {
        Box::new( async move {
            info!("Waiting on the button");
            caller.data_mut().btn.wait_for_low().await;
            info!("Got it ");
            })
    })?;

    let instance = linker.instantiate_async(&mut store, &module).await?;

    instance.get_typed_func::<(), ()>(&mut store, "button_blinky")?.call_async(&mut store, ()).await?;



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