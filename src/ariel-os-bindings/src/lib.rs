#![no_std]

#[cfg(any(feature = "wasm-client", feature = "wasm-host"))]
pub mod wasm;