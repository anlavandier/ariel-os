#![no_std]

use core::panic::PanicInfo;
extern crate alloc;

use talc;

#[global_allocator]
static ALLOCATOR: talc::Talck<talc::locking::AssumeUnlockable, talc::ClaimOnOom> = {
    // Static 4KiB Arena
    static mut MEMORY: [u8; 1024] = [0; 1024];
    let span = talc::Span::from_array(core::ptr::addr_of!(MEMORY).cast_mut());
    talc::Talc::new(unsafe { talc::ClaimOnOom::new(span) }).lock()
};

#[link(wasm_import_module="host")]
unsafe extern "C" {
    fn host_hello();
    fn log(ptr_len: u64);
}

#[unsafe(export_name = "hello_from_host")]
fn call_host_hello() {
    unsafe { host_hello() };
}

#[unsafe(export_name = "static_alloc_and_log")]
fn alloc_and_log() {
    let static_ref: &'static str = "Hello World!";
    let (ptr, len) = (static_ref.as_ptr(), static_ref.len());
    let ptr_len = (ptr as u64) << 32 | len as u64;
    unsafe { log(ptr_len) };
}
#[unsafe(export_name = "dyn_alloc_and_log")]
fn dyn_alloc_and_log(size: u32) {
    let string = "&".repeat(size as usize);
    let (ptr, len) = (string.as_ptr(), string.len());
    let ptr_len = (ptr as u64) << 32 | len as u64;
    unsafe { log(ptr_len) };
    drop(string);
}

#[panic_handler]
fn abort_on_panic(_msg: &PanicInfo) -> ! {
    core::arch::wasm32::unreachable();
}