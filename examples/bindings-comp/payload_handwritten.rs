#![no_std]

use ariel_os_bindings::wasm::rng::hand_written::client_api::{ClientRng, rand_core};
use rand_core::RngCore;


#[unsafe(export_name = "start")]
pub extern "C" fn start() -> u64 {
    let mut myrng = ClientRng;
    let a = myrng.next_u32();
    let b = myrng.next_u64();
    let mut arr = [0u8; 10];
    myrng.fill_bytes(&mut arr);
    a as u64 + b + arr.iter().sum::<u8>() as u64
}

use core::panic::PanicInfo;
#[panic_handler]
fn abort_on_panic(_msg: &PanicInfo) -> ! {
    core::arch::wasm32::unreachable();
}