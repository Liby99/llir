//! LLVM Values
//!
//! LLVM Values has the following hierarchy
//!
//! - [Function](struct.Function.html)
//! - [Global](enum.Global.html)
//! - [Block](struct.Block.html)
//! - [Operand](enum.Operand.html)
//!   - [Instruction](enum.Instruction.html)
//!     - [Alloca](struct.AllocaInstruction.html)
//!     - [Binary](struct.BinaryInstruction.html)
//!       - [Binary Opcode](enum.BinaryOpcode.html) Opcode enum for binary instructions
//!     - [Branch](enum.BranchInstruction.html)
//!       - [Conditional Branch](struct.ConditionalBranchInstruction.html)
//!       - [Unconditional Branch](struct.UnconditionalBranchInstruction.html)
//!     - [Call](struct.CallInstruction.html)
//!     - [CallBr](struct.CallBrInstruction.html)
//!     - [ExtractValue](struct.ExtractValueInstruction.html)
//!     - [FCmp](struct.FCmpInstruction.html)
//!       - [FCmp Predicate](enum.FCmpPredicate.html) Floating point comparison predicate for fcmp instructions
//!     - [Get Element Pointer](struct.GetElementPtr.html)
//!     - [ICmp](struct.ICmpInstruction.html)
//!       - [ICmp Predicate](enum.ICmpPredicate.html) Integer comparison predicate for icmp instructions
//!     - [IndirectBranch](struct.IndirectBranchInstruction.html)
//!     - [InsertValue](struct.InsertValueInstruction.html)
//!     - [Load](struct.LoadInstruction.html)
//!     - [PHI](struct.PhiInstruction.html)
//!     - [Return](struct.ReturnInstruction.html)
//!     - [Select](struct.SelectInstruction.html)
//!     - [Store](struct.StoreInstruction.html)
//!     - [Switch](struct.SwitchInstruction.html)
//!     - [Unary](struct.UnaryInstruction.html)
//!       - [Unary Opcode](enum.UnaryOpcode.html) Opcode enum for unary instructions
//!     - [Unreachable](struct.UnreachableInstruction.html)
//!   - [Constant](enum.Constant.html)
//!     - [Function](struct.Function.html)
//!     - [Global](enum.Global.html)
//!       - [GlobalVariable](enum.GlobalVariable.html)
//!       - [GlobalAlias](enum.GlobalAlias.html)
//!     - [Integer](struct.IntConstant.html)
//!     - [Float](struct.FloatConstant.html)
//!     - [Null](struct.NullConstant.html)
//!     - [Struct](struct.StructConstant.html)
//!     - [Array](struct.ArrayConstant.html)
//!     - [Vector](struct.VectorConstant.html)
//!     - [BlockAddress](struct.BlockAddress.html)
//!     - [Undef](struct.Undef.html)
//!     - [Constant Expression](enum.ConstExpr.html)
//!       - [Binary](struct.BinaryConstExpr.html)
//!         - [Binary Opcode](enum.BinaryOpcode.html)
//!       - [FCmp](struct.FCmpConstExpr.html)
//!         - [FCmp Predicate](enum.FCmpPredicate.html)
//!       - [Get Element Pointer](struct.GetElementPtrConstExpr.html)
//!       - [ICmp](struct.ICmpConstExpr.html)
//!         - [ICmp Predicate](enum.ICmpPredicate.html)
//!       - [Unary](struct.UnaryConstExpr.html)
//!         - [Unary Opcode](enum.UnaryOpcode.html)
//!   - [Argument](struct.Argument.html)
//!   - [Inline Assembly](struct.InlineAsm.html)
//!   - [Metadata](enum.Metadata.html)
//!
//! ## Function & Global
//!
//! You can get `Function`s & `Global`s from Module:
//! ``` rust
//! for func in module.iter_functions() {
//!   // Do things to function
//! }
//! for glob in module.iter_globals() {
//!   // Do things to global
//! }
//! ```
//!
//! ## Instruction
//!
//! You can get instructions by iterating through blocks
//! ``` rust
//! for block in func.iter_blocks() {
//!   for instr in block.iter_instructions() {
//!     // Do things to instruction...
//!   }
//! }
//! ```
//!
//! You can match the instruction with any particular kind of instruction by using pattern
//! matching. From there you can access the instruction specific accessors. As an example,
//! here's how you get the call instruction specific accessor:
//!
//! ```
//! match instr {
//!   Instruction::Call(call) => {
//!     let callee = call.callee_function();
//!     // Do things to the callee function...
//!   }
//! }
//! ```
//!
//! All instructions inherits methods like `operand`, `num_operands`, `parent_block`,
//! `next_instruction` and so. Checkout [InstructionTrait](trait.InstructionTrait.html).
//!
//! ## Operand
//!
//! You can [get operand](trait.InstructionTrait.html#method.operand) from instructions
//!
//! ``` rust
//! let operand_0 = instr.operand(0).unwrap()
//! ```
//!
//! To check which type of operand this is, you do a pattern matching
//!
//! ``` rust
//! match op {
//!   Operand::Constant(cons) => {
//!     // Do things to constant
//!   }
//!   Operand::Instruction(instr) => {
//!     // Do things to instruction
//!   }
//!   // ...
//! }
//! ```
//!
//! Metadata is also a kind of operand, as its used in some intrinsic instructions.
//!
//! ## Constant
//!
//! Constant is also a enum, to get a specific kind of constant, you also do pattern
//! matching
//!
//! ``` rust
//! match my_constant {
//!   Constant::ConstExpr(const_expr) => {
//!     match const_expr {
//!       ConstExpr::GetElementPtr(gep_const_expr) => {
//!         // Get the integer constant indices from the GEP constant expression
//!         let indices = gep_const_expr.int_indices();
//!         // ...
//!       }
//!     }
//!   },
//!   Constant::Function(f) => { /* ... */ },
//!   Constant::Global(g) => { /* ... */ },
//!   // ...
//! }
//! ```
//!
//! Constant expression is a kind of constant. There are only a limited amount of
//! instruction opcode that can apply to constant expressions. To get each different
//! kinds of constant expressions, you should do another pattern matching.
//!
//! Functions & Globals are also constants.
//!
//! Other constants involve integer, float, null and so on.

#[macro_use]
mod macros;
pub use macros::*;

#[macro_use]
mod instruction;
pub use instruction::*;

#[macro_use]
mod constant;
pub use constant::*;

mod function;
mod block;
mod operand;
mod metadata;
mod global;
mod traits;
mod generic;
mod argument;
mod inline_asm;

pub use function::*;
pub use block::*;
pub use operand::*;
pub use metadata::*;
pub use global::*;
pub use traits::*;
pub use generic::*;
pub use argument::*;
pub use inline_asm::*;
