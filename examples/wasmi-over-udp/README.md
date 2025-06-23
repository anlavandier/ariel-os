# wasmi-over-udp

## About

This application is a test of the possibility to run the wasmi interpreter on boards supported by `ariel-os`
and inject a small `wasm` payload with `udp` using the script `send_files.rs`. The `wasm` interpreter expects and tries to run a `main` function that is exposed by the payload with signature `u32 -> u32`. Below is an example of a file that would compile to a compliant payload
```rust
#![no_std]

use core::{hint, panic::PanicInfo};

#[unsafe(export_name = "main")]
extern "C" fn my_main(n: u32) -> u32 {
    let mut result = 0;
    for i in 0..n {
        result = hint::black_box( result + i);
    }
    result
}

#[panic_handler]
fn stall_on_panic(_msg: &PanicInfo) -> ! {
    loop { }
}
```
It can compiled by following these [instructions](https://surma.dev/things/rust-to-webassembly/). It boils down to declaring a lib crate with
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
This has only been tested on the `nrf52840fk`. `wasmi` needs an allocator and atomics to compile

## How to run

In this directory, run

    laze build -b nrf52840dk run

With the device USB cable connected, a USB Ethernet device should pop up.
After configuring the network, the payload can be sent using the [`send_files.rs`](send_files.rs) script.
```bash
cargo +nightly -Zscript send_files.rs -h
warning: `package.edition` is unspecified, defaulting to `2024`
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/tribe/.cargo/target/4e/ecf3aa8cb2bd9e/debug/send_files -h`
Usage: send_files [OPTIONS] --payload <PAYLOAD>

Options:
  -z, --payload <PAYLOAD>          path to wasm payload
  -s, --packet-size <PACKET_SIZE>  packetization level [default: 128]
  -i, --dest-ip <DEST_IP>          Destination IP [default: 10.42.0.61]
  -p, --dest-port <DEST_PORT>      Destination Port [default: 1234]
  -h, --help                       Print help
  -V, --version                    Print version
```

Look [here](../README.md#networking) for more information about network configuration.

