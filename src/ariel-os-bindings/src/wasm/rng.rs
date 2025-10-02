use wasmtime::component::bindgen;


bindgen!({
    world: "rng",
    path: "wit/",
});


pub use ariel::wasm_bindings::rng_api::{Host, HostWithStore, add_to_linker, HostRNG, RNG};