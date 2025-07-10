## Auto Generating APIs

This branch features the API auto generation proposals.
Currently, I'm thinking of having basically two sides

### A [`[wasm_bindgen]`](https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html) like proc macro

For `extern "C" { }` blocks of functions of functions imported from the host :
```rust

#[ariel-os-bindgen(module_name = "host")]
unsafe extern "C" {
  log(input: &str);
}

// Would emit somthing like
#[link(wasm_module = "host")]
unsafe extern "C" {
  __log_generated(ptr: u32, len: u32);
}

fn log(input: &str) {
  unsafe { log(input.as_ptr() as u32, input.len() as u32) };
}
```
For `extern "C" fn` that are exported by the WASM client to be used by the host
```rust
#[ariel-os-bindgen]
extern "C" fn do_something(input: &str){
  do_stuff_with_str(input)
}

// Would emit something like
#[unsafe(export_name = "do_something")]
extern "C" fn __do_something_generated(input_ptr: u32, input_len: u32) {
  let input = unsafe { core::str::from_raw_parts(input_ptr as *const u8, input_len as usize) };
  return do_something(input);
}

do_something(input: &str) {
  do_stuff_with_str(input)
}
```
From the side of the host manage to do something similar.
```rust
#[ariel-os-bindgen-host(wamsi)]
extern "wasm-client" {
  fn do_something(&str);
}

// This would generated something that would allow it to be used like this

fn main {
  // Wasmi stuff
  // let instance: Instance = ...;
  let func_call_result: Result<(), BindError> = do_something(instance, param);
}

// Might need to look someting like that which could obliterate code size....
do_something<T>(instance: Instance<T>, param: &str) -> Result<(), BindError> {
  // see if it's possible to allocate something if so, fill the
  let ptr_addr: u32 = match instance.get_typed_func<(u32, u32, u32), u32>("alloc") {
    Some(alloc_func) => {
      let (size,  align) = ::core::alloc::size_align<str>()
      let len = param.len();
      match alloc_func.call(instance.as_ctx_mut(),(size, align, len))? {
        0 => {
          return BindError::Oom
        },
        addr => {
          match instance.get_export("memory") => {
            Some(Extern::Memory(mem))=> {
              mem.data(instance.as_ctx()).get(addr as usize..(addr + len as usize)).copy_from_slice(param.as_bytes());
              // Return allocation addr
              addr
            },
            _ => unreachable!()
          }
        }
      }
    },
    None => return BindError::NoAlloc,
  }

  // Now call the do_something_host_function with ptr + len
  match instance.get_typed_func<(u32, u32), ()>("do_something") {
    None => return BindError::NoExport,
    Some(func) => {
      func.call((addr, params.len() as u32))
    }
  }

  // Now free the memory
  match instance.get_typed_func<(u32, u32), u32>("free") {
    None => {}
    Some(free_func) => {
      free_func.call(instance.as_ctx_mut(), (addr, alloc_len))?;
    }
  }
}

// Could also have something like this maybe
struct DoSomething;
impl DoSomething {
  fn bind(instance: &Instance) -> Result<impl Fn(&str) -> Result<(), wasmi::Error>, BindError> {
    // Check for alloc, free and the "do_something" function
    return |params: &str| {
      // Actual stuff
    }
  }
}

// Or everything regrouped
#[ariel-os-bindgen-host]
extern "C" {
  fn do_something(&str)
  fn do_other_thing(&mut u32);
}

fn bind_to_host<Params, Results>(name: &str, host: &Instance) -> Result<impl Fn(&str) -> Result<(), wasmi::Error>, BindError> {
  // Common checks maybe
  match name =>
    "do_something" => {return }
    "do_other_thing" => {}
}
```

### A prelude with APIs people are probably going to use

```rust
/// Host Side
use ariel_os::bindings::wasm::host::net::SendPacketBinding;

fn main() {
  // wamsi stuff
  let mut linker = Linker::new();
  SendPacketBinding::bind(linker);

}

/// Client Side
use ariel_os::bindings::wasm::client::net::{send_packet, Packet};


fn SendMyPacket() {
  let p = Packet::default();
  send_packet(Packet);
}
```