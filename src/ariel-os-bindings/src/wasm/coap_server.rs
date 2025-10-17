use wasmtime::component::bindgen;

#[cfg(feature = "async")]
pub mod with_async {
    use super::bindgen;
    bindgen!({
        world: "coap-server",
        path: "wit/",

        imports: { "ariel:wasm-bindings/coap-server-api/run": async},
    });

}

#[cfg(feature = "async")]
pub use with_async::ariel::wasm_bindings;


#[cfg(not(feature = "async"))]
pub mod sync {
    use super::bindgen;
    bindgen!({
        world: "coap-server",
        path: "wit/",
    });

}

#[cfg(not(feature = "async"))]
pub use sync::ariel::wasm_bindings;

use ariel_os_embassy::cell::SameExecutorCell;

pub use wasm_bindings::coap_server_api::{Host, HostWithStore, add_to_linker, CoapReply};

extern crate alloc;
use alloc::{boxed::Box, string::String};

use coap_handler_implementations::{HandlerBuilder, SimpleRendered, new_dispatcher};
use coap_handler::{Reporting, Handler};


trait MyTrait: Handler + Reporting + Clone {}

pub struct ArielCoapServerHost {
    handler: Box<dyn MyTrait>,
}


impl Default for ArielCoapServerHost {
    fn default() -> Self {
        ArielCoapServerHost { handler: Box::new(
            new_dispatcher()) }
    }
}

impl Host for ArielCoapServerHost {
    fn register(&mut self, at: String, response: CoapReply) {
        self.handler = match response {
            CoapReply::S(reply) => {
                let h_clone = self.handler.clone();
                HandlerBuilder::at(*h_clone, &[&at], SimpleRendered(reply.as_str()))
            },
        }
    }


    // Block the wasm forever because this never returns but doesn't block the host...
    #[cfg(feature = "async")]
    async fn run(&mut self) {
        ariel_os_coap::coap_run(self.handler).await;

    }

    #[cfg(not(feature = "async"))]
    fn run(&mut self) {
        todo!()
    }
}
