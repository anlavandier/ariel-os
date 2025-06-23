# wasmi-example

## About

This application is a test of the possibility to run the wasmi interpreter on Board supported by Ariel
The `wasm` is present in textual form in [`ex.wat`](ex.wat) and in binary form in `low_mem_example.wasm`.
They were compiled from single `lib.rs` containing :
```rust
#![no_std]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
extern "C" fn add(left: u32, right: u32) -> u32 {
    unsafe { host_hello(32); };
    left + right
}

#[link(wasm_import_module = "host")]
unsafe extern "C" {
    fn host_hello(i: u32);
}


#[panic_handler]
fn stall_on_panic(_msg: &PanicInfo) -> ! {
    loop { }
}
```
by following these [instructions](https://surma.dev/things/rust-to-webassembly/). It boils down to declaring a lib crate with
```toml
[lib]
crate-type = ["cdylib", "rlib"]
```
and building it with
```bash
cargo rustc --target=wasm32-unknown-unknown --release -- -Clink-arg=-zstack-size=16 -Clink-arg=--initial-memory=65536
```
The stack size needs to be changed because by default it is set to `1MB` and thus requires the `wasm` programm to also ask for `1MB` of inital memory which is more than the board has.
The initial and max memory need to be multiples of 65536 bytes by default. There is some support in the `wasm` specs (and in `wasmi`) for non standard page size but this hasn't been tested yet.
This has only been tested on the `nrf52840fk`. `wasmi` needs an allocator and atomics to compile.

## How to run

In this directory, run

    laze build -b nrf52840dk run
