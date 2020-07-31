# LLVM IR Analysis in Rust

How to use:

``` rust
use llir;

// Create context
let context = llir::Context::create();

// Specify path to the byte code
let path = Path::new("tests/c_files/fn_ptr/fn_ptr_1.bc");

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