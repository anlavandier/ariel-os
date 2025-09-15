#![no_main]
#![no_std]

use ariel_os::debug::log::info;
use embassy_futures::block_on;
use embedded_nal_coap::CoAPRuntimeClient;
use wasmtime::{AsContext, Caller, Config, Engine, Extern, Instance, Linker, Module, Store};

use coap_request::Stack;

#[ariel_os::task(autostart)]
async fn run_client_operations() {

    let engine = set_up_wasm();
    let mut store = Store::new(&engine, ariel_os::coap::coap_client().await);

    let i = start_wasm(&engine, &mut store);

    i.get_typed_func::<(), ()>(&mut store, "POST_demo").unwrap().call(&mut store, ()).unwrap();
    i.get_typed_func::<(), ()>(&mut store, "GET_demo").unwrap().call(&mut store, ()).unwrap();
}

fn set_up_wasm() -> Engine {
    let mut config = Config::default();
    config.max_wasm_stack(2048);
    config.wasm_custom_page_sizes(true);
    config.target("pulley32").unwrap();
    config.memory_reservation(0);
    config.memory_init_cow(false);
    config.memory_reservation_for_growth(0);
    config.memory_may_move(false);
    let engine = Engine::new(&config).unwrap();
    engine
}

fn start_wasm(
    engine: &Engine,
    store: &mut Store<&'static CoAPRuntimeClient<'_, 3>>,
) -> Instance {
    let module_bytes = include_bytes!("../coap-client-demo.cwasm");
    let module = match unsafe { Module::deserialize_raw(&engine, module_bytes.as_slice().into())} {
        Ok(module) => module,
        Err(err) => {
            info!("Error Initiliaing wasm: {:?}", defmt::Debug2Format(&err));
            panic!();
        }
    };

    let mut linker = Linker::new(&engine);

    linker.func_wrap("coap-test", "get", |
        mut caller: Caller<'_, _>,
        data_ptr: u32,
        data_len: u32,
        addr_ptr: u32,
        addr_len: u32,
        | {
        let client: &'static CoAPRuntimeClient<'_, 3> = *caller.data();

        let data_ptr = data_ptr as usize;
        let data_len = data_len as usize;
        let addr_ptr = addr_ptr as usize;
        let addr_len = addr_len as usize;
        let mem = match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => mem,
            _ => unreachable!()
        };
        let mem_repr = mem.data(caller.as_context());
        // Assume that the adress is encoded as a string
        let addr = str::from_utf8(&mem_repr[addr_ptr..addr_ptr + addr_len]).unwrap();
        let payload = &mem_repr[data_ptr..data_ptr + data_len];
        let demoserver = addr.parse().unwrap();

        let request = coap_request_implementations::Code::get()
            .with_path("/uppercase")
            .with_request_payload_slice(payload)
            .processing_response_payload_through(|p: &[u8]| {
                info!(
                    "Response is {:?}",
                    core::str::from_utf8(p).map_err(|_| { "Not Unicode ?"})
                );
            });
        let response = block_on(client.to(demoserver).request(request));
        info!("Response: {:?}", response.map_err(|_| "Transport error"));
    }).unwrap();



    linker.func_wrap("coap-test", "post", |
        mut caller: Caller<'_, _>,
        data_ptr: u32,
        data_len: u32,
        addr_ptr: u32,
        addr_len: u32,
        | {
        let client: &'static CoAPRuntimeClient<'_, 3> = *caller.data();

        let data_ptr = data_ptr as usize;
        let data_len = data_len as usize;
        let addr_ptr = addr_ptr as usize;
        let addr_len = addr_len as usize;
        let mem = match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => mem,
            _ => unreachable!()
        };
        let mem_repr = mem.data(caller.as_context());
        // Assume that the adress is encoded as a string
        let addr = str::from_utf8(&mem_repr[addr_ptr..addr_ptr + addr_len]).unwrap();
        let payload = &mem_repr[data_ptr..data_ptr + data_len];
        let demoserver = addr.parse().unwrap();

        let request = coap_request_implementations::Code::post()
            .with_path("/uppercase")
            .with_request_payload_slice(payload)
            .processing_response_payload_through(|p: &[u8]| {
                info!(
                    "Response is {:?}",
                    core::str::from_utf8(p).map_err(|_| { "Not Unicode ?"})
                );
            });
        let response = block_on(client.to(demoserver).request(request));
        info!("Response: {:?}", response.map_err(|_| "Transport error"));
    }).unwrap();
    let instance = linker.instantiate(store, &module).unwrap();
    instance
}

// Same as https://github.com/bytecodealliance/wasmtime/blob/main/examples/min-platform/embedding/wasmtime-platform.c
// I have no idea whether this is safe or not.
// https://github.com/bytecodealliance/wasmtime/blob/aec935f2e746d71934c8a131be15bbbb4392138c/crates/wasmtime/src/runtime/vm/traphandlers.rs#L888
static mut TLS_PTR: u32 = 0;
#[unsafe(no_mangle)]
extern "C" fn wasmtime_tls_get() -> *mut u8 {
    unsafe { TLS_PTR as *mut u8 }
}

#[unsafe(no_mangle)]
extern "C" fn wasmtime_tls_set(val: *const u8) {
   unsafe { TLS_PTR = val as u32 };
}