#![no_std]

use core::{panic::PanicInfo};


// #[ariel-os-bindgen]
#[link(wasm_import_module="log")]
unsafe extern "C" {
    fn log_str(ptr: u32, len: u32);
}


/// Would be automatically generated.
#[inline(always)]
fn log_safe(input: &str) {
    unsafe { log_str(input.as_ptr() as u32, input.len() as u32) };
}

#[unsafe(export_name = "static_alloc_and_log")]
extern "C" fn alloc_log() {
    let hello = "Hello World";
    log_safe(hello);
}

#[panic_handler]
fn abort_on_panic(_msg: &PanicInfo) -> ! {
    core::arch::wasm32::unreachable();
}