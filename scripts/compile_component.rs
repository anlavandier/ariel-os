#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[package]
edition = "2024"

[dependencies]
clap = { version = "4.5.40", features = ["derive"]}
wasmtime = { path = "../../wasmtime/crates/wasmtime", default-features = false, features = ["component-model", "async", "cranelift", "pulley"] }
miette = { version = "7.2.0", features = ["fancy"] }
thiserror = { version = "2.0.12" }

---
#![feature(trim_prefix_suffix)]
use std::{fs, io, path::PathBuf};
use clap::Parser;
use miette::Diagnostic;

use wasmtime::{Config, Engine};


/// Simple CLI that takes a compiled .wasm file and turns into a precompiled-component
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the Wasm file
    #[arg(short, long)]
    path: PathBuf,

    /// Turn fuel instrumentation on
    #[arg(short, long)]
    fuel: bool,

    /// Path of the output file
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Path to wasm-tools if isn't not in $PATH
    #[arg(short, long)]
    wasm_tools: Option<PathBuf>,
}

#[derive(Debug, thiserror::Error, Diagnostic)]
enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Precompilation Error: {0}")]
    Precomp(#[from] wasmtime::Error),
}


fn main() -> miette::Result<()> {
    let args = Args::parse();

    let Args { path, fuel, output, wasm_tools } = args;


    assert!(fs::exists(&path).map_err(Error::from)?);

    // Turn into a component
    let wasm_tools_path = if wasm_tools.is_some() {
        wasm_tools.unwrap()
    } else {
        "wasm-tools".into()
    };
    // Note: On Unix, non UTF8 strings are invalid Paths so the unwrap() is infallible
    std::process::Command::new(wasm_tools_path)
        .args(["component", "new", path.as_path().to_str().unwrap(), "-o", "temp.wasm"])
        .output()
        .map_err(Error::from)?;

    let out = if output.is_some() {
        output.unwrap()
    } else {
        let mut temp: PathBuf = path.file_stem().unwrap().into();
        temp.set_extension("cwasm");
        temp
    };

    precompile("temp.wasm", fuel, out)?;

    Ok(())
}

fn precompile(path: &str, fuel: bool, out: PathBuf) -> miette::Result<()> {

    let mut config = Config::new();
    // Tell how the memory is expected to behave when instantiated
    config.memory_reservation(0);
    config.wasm_custom_page_sizes(true);
    config.memory_may_move(false);
    config.memory_init_cow(false);


    // Optimize for Code size in the .cwasm
    config.cranelift_opt_level(wasmtime::OptLevel::SpeedAndSize);
    config.target("pulley32").map_err(Error::from)?;

    // Enable fuel intstrumentation to prevent malevolent code from running indefinitely in the VM
    config.consume_fuel(fuel);

    // Create an `Engine` with that configuration.
    let engine = Engine::new(&config).map_err(Error::from)?;

    let wasm = fs::read(path).map_err(Error::from)?;

    let precompiled = engine.precompile_component(&wasm).map_err(Error::from)?;

    fs::write(out, &precompiled).map_err(Error::from)?;

    Ok(())
}