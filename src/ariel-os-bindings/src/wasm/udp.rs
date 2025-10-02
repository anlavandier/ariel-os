use wasmtime::component::bindgen;

bindgen!({
    path: "wit/udp.wit",
    world: "udp",
});

// Reexport traits
pub use ariel::wasm_bindings::udp_api::{Host, HostWithStore, HostUdpSocket};

// Reexport structs
pub use ariel::wasm_bindings::udp_api::{
    UdpMetadata, IpAddr, Endpoint, Ipv4Addr, Ipv6Addr, UdpSocket
};


pub use ariel::wasm_bindings::udp_api::add_to_linker;