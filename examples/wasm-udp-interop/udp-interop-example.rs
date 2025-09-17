#![no_std]

const BUFFER_SIZE: usize = 128;

#[link(wasm_import_module = "udp-test")]
unsafe extern "C" {
    fn try_recv(buf_ptr: *mut u8, buf_len: u32, endpoint_ptr: *mut u8) -> u32;
    fn send(buf_ptr: *const u8, buf_len: u32, endpoint_ptr: *const u8);
}

#[unsafe(export_name = "udp_echo_plus_one")]
extern "C" fn udp_echo_plus_one() {
    let mut packet_buf = [0; BUFFER_SIZE];
    let mut endpoint_buf = [0; 6]; // 6 bytes: 4 for the IPv4 addr and 2 for the port
    match unsafe { try_recv(packet_buf.as_mut_ptr(), packet_buf.len() as u32, endpoint_buf.as_mut_ptr()) } {
        0 => {},
        n => {
            for b in packet_buf[..n as usize].iter_mut() {
                *b += 1;
            }
            unsafe { send(packet_buf.as_ptr(), n, endpoint_buf.as_ptr()) };
        }
    }
}

use core::panic::PanicInfo;
#[panic_handler]
fn abort_on_panic(_msg: &PanicInfo) -> ! {
    core::arch::wasm32::unreachable();
}