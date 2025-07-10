use ariel_os_debug::log::{defmt::{self, Display2Format}, info};
use wasmi::{AsContext, Caller, Extern, IntoFunc, WasmTy};

#[must_use]
pub fn log_str_builder<'a, T>() -> impl IntoFunc<T, (Caller<'a, T>, u32, u32), ()> {
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
    }
}

#[must_use]
pub fn log_builder<'a, T, Input: WasmTy + defmt::Format + core::fmt::Display>() -> impl IntoFunc<T, (Caller<'a, T>, Input), ()> {
    |_caller: Caller<'_, T>, a: Input| {
        info!("{}", a);
    }
}