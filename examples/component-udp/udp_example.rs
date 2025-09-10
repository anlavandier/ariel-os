const BUFFER_SIZE: usize = 128;


#[link(wasm_import_module = "udp-test")]
unsafe extern "C" {
    fn blocking_receive_packet(ptr: *mut u8) -> u32;
    fn blocking_send_packet(ptr: *const u8, data_size: u32);
}

#[unsafe(export_name = "do_stuff")]
// Add 1 to every byte of received packet data
extern "C" fn do_stuff() {

    let mut data_buf = [0; BUFFER_SIZE];

    let written_part = unsafe { blocking_receive_packet(data_buf.as_mut_ptr()) };
    if written_part != 0 {
        for byte in data_buf[..written_part as usize].iter_mut() {
            *byte += 1;
        }
        unsafe { blocking_send_packet(data_buf.as_ptr(), written_part) };
    }

}