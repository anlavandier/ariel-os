use ariel_os_debug::log::{defmt::{self, Display2Format}, info};
use wasmi::{AsContext, Caller, Extern, Linker};


/// # Errors
/// - When the linker complains
pub fn export_log<T>(linker: &mut Linker<T>) -> Result<&mut Linker<T>, wasmi::errors::LinkerError>{
    linker.func_wrap("log", "log_str",
    |caller: Caller<'_, T>, str_ptr:u32, str_len: u32| {
        let ptr = str_ptr as usize;
        let len = str_len as usize;
        match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => {
                let str_slice = mem.data(caller.as_context()).get(ptr..(ptr + len)).unwrap_or_default();
                match str::from_utf8(str_slice) {
                    Ok(utf8_str) => info!("{}", utf8_str),
                    Err(e) => info!("{}", Display2Format(&e)),
                }
            },
            _ => unreachable!(),
        }
    })
}
