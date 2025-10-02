use wasmtime::component::bindgen;

#[cfg(feature = "async")]
bindgen!({
    world: "time",
    path: "wit/",

    imports: {
        "ariel:wasm-bindings/time-api/sleep": async,
    }
});

#[cfg(not(feature = "async"))]
bindgen!({
    world: "time",
    path: "wit/",
});
pub use ariel::wasm_bindings::time_api::{Host, HostWithStore, add_to_linker};