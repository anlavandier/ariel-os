#![no_main]
#![no_std]

use ariel_os::coap::coap_run;
use ariel_os::debug::{log::{info, defmt}};

use coap_numbers::code::NOT_FOUND;
use wasmtime::{AsContext, Caller, Config, Engine, Extern, Linker, Module, Store};


#[ariel_os::task(autostart)]
async fn main() {
    run_wasm_coap_server().await.unwrap();
}




async fn run_wasm_coap_server() -> wasmtime::Result<()> {
    let mut config = Config::default();
    config.max_wasm_stack(2048);
    config.wasm_custom_page_sizes(true);
    config.target("pulley32").unwrap();
    config.memory_reservation(0);
    config.memory_init_cow(false);
    config.memory_reservation_for_growth(0);
    config.memory_may_move(false);
    let engine = Engine::new(&config).unwrap();


    let mut store = Store::new(&engine, ());

    let wasm = include_bytes!("../coap-server-impl.cwasm");
    // let wasm = [];

    #[allow(unsafe_code)]
    let component = unsafe { Module::deserialize_raw(&engine, wasm.as_slice().into())? };

    let mut linker = Linker::new(&engine);

    linker.func_wrap("log","info", |mut caller: Caller<'_, _>, str_ptr: u32, str_len: u32| {
        match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => {
                #[allow(unsafe_code)]
                let wasm_str = unsafe { core::str::from_utf8_unchecked(&mem.data(caller.as_context())[str_ptr as usize .. str_ptr as usize + str_len as usize]) };
                info!("[WASM] {}", wasm_str);
            },
            _ => unreachable!("should be a memory"),
        }
    })?;

    linker.func_wrap("log","dump_buffer", |mut caller: Caller<'_, _>, ptr: u32, len: u32| {
        match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => {
                let buffer = &mem.data(caller.as_context())[ptr as usize .. ptr as usize + len as usize];
                info!("[WASM] len: {:?}, {:?}", buffer.len(), buffer);
            },
            _ => unreachable!("should be a memory"),
        }
    })?;

    let instance = linker.instantiate(&mut store, &component)?;

    let handler = build_wasm_handler(store, instance);
    info!("Starting Handler");
    coap_run(handler).await;
}

// Same as https://github.com/bytecodealliance/wasmtime/blob/main/examples/min-platform/embedding/wasmtime-platform.c
// I have no idea whether this is safe or not.
// https://github.com/bytecodealliance/wasmtime/blob/aec935f2e746d71934c8a131be15bbbb4392138c/crates/wasmtime/src/runtime/vm/traphandlers.rs#L888
static mut TLS_PTR: u32 = 0;

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
extern "C" fn wasmtime_tls_get() -> *mut u8 {
    #[allow(unsafe_code)]
    unsafe { TLS_PTR as *mut u8 }
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
extern "C" fn wasmtime_tls_set(val: *const u8) {
    #[allow(unsafe_code)]
    unsafe { TLS_PTR = val as u32 };
}


use coap_handler::{Handler, Reporting};
use coap_handler_implementations::{new_dispatcher, HandlerBuilder, ReportingHandlerBuilder, SimpleRendered};
use coap_message_implementations::inmemory_write::GenericMessage;
use coap_message::MessageOption;

pub struct WasmHandlerTest<T: 'static> {
    store: wasmtime::Store<T>,
    instance: wasmtime::Instance,
    coap_buffer_info: usize
}


/// FIXME: use trait function when the WithSortedOptions bound
mod disable_sort_options_bound {
    use coap_message::MessageOption;
    use coap_message::{Code, OptionNumber};

    pub trait AbleToBeSetFromMessage: coap_message::MinimalWritableMessage {

        fn set_from_message2<M>(&mut self, msg: &M) -> Result<(), Self::UnionError>
        where
            M: coap_message::ReadableMessage,
        {


            self.set_code(Self::Code::new(msg.code().into())?);

            for opt in msg.options() {
                self.add_option(Self::OptionNumber::new(opt.number())?, opt.value())?;
            }
            self.set_payload(msg.payload())?;
            Ok(())
        }
    }

    impl<T: coap_message::MinimalWritableMessage> AbleToBeSetFromMessage for T { }
}

use disable_sort_options_bound::AbleToBeSetFromMessage;

impl<T: 'static> Handler for WasmHandlerTest<T> {
    // request data is just the code
    type RequestData = u8;

    type ExtractRequestError = coap_message_utils::Error;

    type BuildResponseError<M: coap_message::MinimalWritableMessage> = M::UnionError;


    fn extract_request_data<M: coap_message::ReadableMessage>(
            &mut self,
            request: &M,
        ) -> Result<Self::RequestData, Self::ExtractRequestError> {
        info!("Got a request!");
        let mut incoming_code: u8 = request.code().into();
        info!("HOST incoming request with payload {:?}", request.payload());
        for o in request.options() {info!("HOST Option {} {:?}", o.number(), o.value());};
        let buf_addr = self.coap_buffer_info;
        let mem = self.instance.get_memory(&mut self.store, "memory")
            .expect("Instances should always have memories");
        let buffer = &mut mem
            .data_mut(&mut self.store)[buf_addr..buf_addr + 1200];

        let mut reencoded = GenericMessage::new(&mut incoming_code, buffer);
        reencoded.set_from_message2(request).unwrap();
        let incoming_len = reencoded.finish();

        info!("HOST len: {}\n {:?}", incoming_len, buffer);

        let new_code = match self.instance.get_typed_func::<(u32, u32), u32>(&mut self.store, "coap_run")
            .expect("required for the coap-server-impl")
            .call(&mut self.store, (incoming_code as u32, incoming_len as u32)) {
                Ok(c) => { c },
                Err(e) => {
                    info!("{:?}", defmt::Debug2Format(&e));
                    info!("Will panic in a few moments");
                    for _ in 0..1_000_000 { core::hint::black_box(()) }
                    panic!();
                },
            };
        match new_code.try_into() {
            Err(_) => {
                unreachable!("On the wasm side a u8 is turned into a u32");
            }
            Ok(NOT_FOUND) => {
                Err(coap_message_utils::Error::not_found())
            },
            Ok(new_code) => {
                Ok(new_code.try_into().unwrap())
            }
        }


    }

    fn estimate_length(&mut self, _request: &Self::RequestData) -> usize {
        1200
    }

    fn build_response<M: coap_message::MutableWritableMessage>(
            &mut self,
            response: &mut M,
            request: Self::RequestData,
        ) -> Result<(), Self::BuildResponseError<M>> {

        let buf_addr = self.coap_buffer_info;
        let mem = self.instance.get_memory(&mut self.store, "memory")
            .expect("Instances should always have memories");
        let buffer = &mem
            .data(&self.store)[buf_addr..buf_addr + 1200];

        response.set_from_message2(&coap_message_implementations::inmemory::Message::new(request, buffer))
    }
}

use coap_handler_implementations::wkc;
impl<T: 'static> Reporting for WasmHandlerTest<T> {
    type Record<'a>
        = wkc::EmptyRecord
    where
        Self: 'a;
    type Reporter<'a>
        = core::iter::Once<wkc::EmptyRecord>
    where
        Self: 'a;

    fn report(&self) -> Self::Reporter<'_> {
        // Using a ConstantSliceRecord instead would be tempting, but that'd need a const return
        // value from self.0.content_format()
        core::iter::once(wkc::EmptyRecord {})
    }
}


fn build_wasm_handler<T:'static>(mut store: wasmtime::Store<T>, instance: wasmtime::Instance) -> impl Handler + Reporting {

    let coap_buffer_info = instance.get_typed_func::<(u32,), u32>(&mut store, "alloc_coap_buffer")
        .expect("The Capsule needs to provide a buffer for CoAP")
        .call(&mut store, (1200,)).unwrap();

    let handler = new_dispatcher()
        .below(&["vm"], WasmHandlerTest { store, instance, coap_buffer_info: coap_buffer_info as usize })
        .at(&["hello"], SimpleRendered("Hello from the host"))
        .with_wkc();

    return handler;
}
