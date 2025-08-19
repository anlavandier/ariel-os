pub mod hand_written {
    #[cfg(feature = "wasm-host")]
    pub mod host_api {
        use ariel_os_random::fast_rng;
        use rand_core::RngCore;
        use wasmtime::{Linker, AsContextMut, Caller, Extern};

        /// # Errors
        /// - When the linker complains
        pub fn export_rng<T: 'static>(linker: &mut Linker<T>) -> Result<&mut Linker<T>, wasmtime::Error>{
            linker.func_wrap("rng", "__next_u32", || {
                let mut frng=  fast_rng();
                frng.next_u32()
            })?;
            linker.func_wrap("rng", "__next_u64", || {
                let mut frng=  fast_rng();
                frng.next_u64()
            })?;

            linker.func_wrap("rng", "__fill_bytes", |mut caller: Caller<'_,T>, ptr: u32, len: u32| {
                if let Some(Extern::Memory(mem)) = caller.get_export("memory") {
                    let slice_ptr = ptr as usize;
                    let slice_len = len as usize;
                    if let Some(b_slice) = mem.data_mut(caller.as_context_mut()).get_mut(slice_ptr..slice_ptr + slice_len) {
                        let mut frng=  fast_rng();
                        frng.fill_bytes(b_slice);
                    }
                }
            })?;
            linker.func_wrap("rng", "__try_fill_bytes", |mut caller: Caller<'_,T>, ptr: u32, len: u32| {
                if let Some(Extern::Memory(mem)) = caller.get_export("memory") {
                    let slice_ptr = ptr as usize;
                    let slice_len = len as usize;
                    if let Some(b_slice) = mem.data_mut(caller.as_context_mut()).get_mut(slice_ptr..slice_ptr + slice_len) {
                        let mut frng=  fast_rng();
                        let res = frng.try_fill_bytes(b_slice);
                        u32::from(res.is_ok())
                    }
                    else {
                        u32::from(false)
                    }
                }
                else {
                    u32::from(false)
                }
            })?;
            Ok(linker)
        }
    }

    #[cfg(feature = "wasm-client")]
    pub mod client_api {

        use core::num::NonZeroU32;

        pub use rand_core;


        #[link(wasm_import_module = "rng")]
        unsafe extern "C" {
            fn __next_u32() -> u32;
            fn __next_u64() -> u64;
            fn __fill_bytes(ptr: u32, len:u32);
            fn __try_fill_bytes(ptr: u32, len: u32) -> u32;
        }

        pub fn _next_u32() -> u32 {
            unsafe { __next_u32() }
        }
        pub fn _next_u64() -> u64 {
            unsafe { __next_u64() }
        }
        pub fn _fill_bytes(dest: &mut [u8]) {
            unsafe { __fill_bytes(dest.as_mut_ptr() as u32, dest.len() as u32) };
        }
        pub fn _try_fill_bytes(dest: &mut [u8]) -> Result<(), rand_core::Error> {
            let success = unsafe { __try_fill_bytes(dest.as_mut_ptr() as u32, dest.len() as u32) } > 0;
            success.then_some(()).ok_or(unsafe { NonZeroU32::new_unchecked(1) }.into())
        }

        pub struct ClientRng;

        impl rand_core::RngCore for ClientRng {
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                _fill_bytes(dest);
            }
            fn next_u32(&mut self) -> u32 {
                _next_u32()
            }
            fn next_u64(&mut self) -> u64 {
                _next_u64()
            }
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
                _try_fill_bytes(dest)
            }
        }

    }
}

#[cfg(feature = "wasm-host")]
pub mod with_component_model {
    use ariel_os_random::fast_rng;
    use rand_core::RngCore;
    use wasmtime::component::bindgen;
    extern crate alloc;
    use alloc::vec::Vec;

    bindgen!({
        path: "wit/rng.wit",
        world: "rng",
    });

    pub use ariel::bindings::rng_api::Host;

    #[derive(Debug, Default)]
    pub struct ArielRngHost;

    impl Host for ArielRngHost {
        fn next_u32(&mut self,) -> u32 {
            let mut frng = fast_rng();
            frng.next_u32()
        }
        fn next_u64(&mut self,) -> u64 {
            let mut frng = fast_rng();
            frng.next_u64()
        }
        fn random_bytes(&mut self,len:u32,) -> wasmtime::component::__internal::Vec<u8> {
            let mut v = core::iter::repeat_n(0_u8, len as usize).collect::<Vec<_>>();
            let mut frng = fast_rng();
            frng.fill_bytes(&mut v);
            v
        }
    }
    pub use ariel::bindings::rng_api::add_to_linker;
}