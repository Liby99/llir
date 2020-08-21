# LLVM IR Binding for Rust

This LLVM IR Binding provides intuitive and well-organized safe Rust API for analyzing existing LLVM modules.
Thus the whole library is thread-safe. This crate does not provide the functionality to produce new LLVM module
or change existing module.

## How to use

``` rust
use llir;

// Create context
let context = llir::Context::create();

// Specify path to the byte code
let path = Path::new("path/to/your/llvm/bytecode.bc");

// Load the module with that path
let module = context.load_module(path)?;

// Iterate through functions, blocks, and instructions...
for func in module.iter_functions() {
  for block in func.iter_blocks() {
    for instr in block.iter_instructions() {
      // Do things to instr...
    }
  }
}
```

## Documentation

The automatically generated documentation is hosted [on docs.rs](https://docs.rs/llir/)

## Include as a library

Make sure you installed LLVM 10.0 on your machine and is visible via path.

Then go to your `Cargo.toml` and add this line under your dependencies:

``` toml
# Cargo.toml
[dependencies]
llir = "0.1"
```