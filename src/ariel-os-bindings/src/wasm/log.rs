use wasmtime::component::bindgen;

bindgen!({
    world: "log",
    path: "wit/",
});


pub use ariel::wasm_bindings::log_api::{Host, HostWithStore, add_to_linker};