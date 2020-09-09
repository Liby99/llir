//! # LLVM IR Binding for Rust
//!
//! This LLVM IR Binding provides intuitive and well-organized safe Rust API for analyzing existing LLVM modules.
//! Thus the whole library is thread-safe. This crate does not provide the functionality to produce new LLVM module
//! or change existing module.
//!
//! ## Getting started
//!
//! ``` rust
//! use llir;
//!
//! // Create context
//! let context = llir::Context::create();
//!
//! // Specify path to the byte code
//! let path = Path::new("path/to/your/llvm/bytecode.bc");
//!
//! // Load the module with that path
//! let module = context.load_module(path)?;
//!
//! // Iterate through functions, blocks, and instructions...
//! for func in module.iter_functions() {
//!   for block in func.iter_blocks() {
//!     for instr in block.iter_instructions() {
//!       // Do things to instr...
//!     }
//!   }
//! }
//! ```
//!
//! Other than context and module, the crate mainly consists of LLVM values and types. Checkout each
//! submodule for more information.

#[macro_use]
mod utils;

mod context;
mod module;
pub mod types;
pub mod values;

pub use context::*;
pub use module::*;
pub use utils::traits::*;
