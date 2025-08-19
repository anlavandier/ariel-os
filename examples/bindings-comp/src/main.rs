#![no_main]
#![no_std]

use ariel_os::{debug::{exit, log::{defmt, info}, ExitCode}, time::Timer};

#[cfg(feature = "handwritten")]
#[path = "handwritten.rs"]
mod wasm_impl;

#[cfg(feature = "with_component_model")]
#[path = "with_component_model.rs"]
mod wasm_impl;

use wasm_impl::run_wasm;





#[ariel_os::task(autostart)]
async fn main() {
    info!(
        "Hello from main()! Running on a {} board.",
        ariel_os::buildinfo::BOARD
    );
    match run_wasm() {
        Ok(s) => info!("Random sum = {}", s),
        Err(e) => info!("{:?}", defmt::Debug2Format(&e)),
    }
    Timer::after_millis(100).await;
    exit(ExitCode::SUCCESS);
}
