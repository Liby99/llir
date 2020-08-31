//! LLVM Types
//!
//! The LLVM Types mainly follows the following hierarchy
//!
//! - [Type](enum.Type.html)
//!   - [Void](struct.VoidType.html)
//!   - [Int](struct.IntType.html)
//!   - [Float](struct.FloatType.html)
//!   - [Pointer](struct.PointerType.html)
//!   - [Array](struct.ArrayType.html)
//!   - [Vector](struct.VectorType.html)
//!   - [Struct](enum.StructType.html)
//!     - [Named Struct](struct.NamedStructType.html)
//!     - [Struct Literal](struct.LiteralStructType.html)
//!   - [Function](struct.FunctionType.html)
//!
//! ## How to use
//!
//! You can get a type from a valid value, for example a function
//!
//! ``` rust
//! for func in module.iter_functions() {
//!   let func_pointer_type = func.get_type();
//!   match func_pointer_type {
//!     Type::PointerType(p) => {
//!       match p.element_type() {
//!         Type::FunctionType(func_type) => {
//!           let return_type = func_type.return_type();
//!           let argument_types = func_type.argument_types();
//!           // Do things to function type...
//!         }
//!         _ => panic!("Type of a function should be a pointer to a function type")
//!       }
//!     }
//!     _ => panic!("Type of a function should be a pointer to a function type")
//!   }
//! }
//! ```
//!
//! You can also get a type from globals, constants, arguments, and part of instructions.
//!
//! Note that instructions like `branch` doesn't contain a type. So we don't provide
//! `get_type()` method for every instruction.

#[macro_use]
mod macros;
pub use macros::*;

mod types;
mod void;
mod int;
mod float;
mod pointer;
mod array;
mod vector;
mod struc;
mod function;
mod traits;
mod generic;

pub use types::*;
pub use void::*;
pub use int::*;
pub use float::*;
pub use pointer::*;
pub use array::*;
pub use vector::*;
pub use struc::*;
pub use function::*;
pub use traits::*;
pub use generic::*;
