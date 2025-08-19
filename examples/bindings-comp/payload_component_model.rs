#![no_std]

#![allow(warnings)]




#[global_allocator]
static ALLOCATOR: talc::Talck<talc::locking::AssumeUnlockable, talc::ClaimOnOom> = {
    // Static 4KiB Arena
    static mut MEMORY: [u8; 4096] = [0; 4096];
    let span = talc::Span::from_array(core::ptr::addr_of!(MEMORY).cast_mut());
    talc::Talc::new(unsafe { talc::ClaimOnOom::new(span) }).lock()
};

mod bindings;

use bindings::{Guest, ariel::bindings::rng_api::{next_u32, next_u64, random_bytes}};

use rand_core::RngCore;

struct ClientRng;

impl RngCore for ClientRng {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.copy_from_slice(&random_bytes(dest.len() as u32));
    }
    fn next_u32(&mut self) -> u32 {
        next_u32()
    }
    fn next_u64(&mut self) -> u64 {
        next_u64()
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}

struct Component;

impl Guest for Component {
    fn start() -> u64 {
        let mut myrng = ClientRng;
        let a = myrng.next_u32();
        let b = myrng.next_u64();
        let mut arr = [0u8; 10];
        myrng.fill_bytes(&mut arr);
        a as u64 + b + arr.iter().sum::<u8>() as u64
    }
}


bindings::export!(Component with_types_in bindings);

#[cfg(target_arch = "wasm32")]
mod panic {
    use core::panic::PanicInfo;
    #[panic_handler]
    pub fn panic_handler(_: &PanicInfo) -> ! {
        core::arch::wasm32::unreachable()
    }
}