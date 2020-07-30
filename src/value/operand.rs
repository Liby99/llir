use llvm_sys::prelude::{LLVMValueRef};
use llvm_sys::core::{LLVMIsAInstruction, LLVMIsAConstant};

use super::{Instruction, Constant, /* ValueRef,*/ FromLLVM};

#[derive(Copy, Clone)]
pub enum Operand<'ctx> {
  Instruction(Instruction<'ctx>),
  Constant(Constant<'ctx>),
  Metadata, // TODO
}

impl<'ctx> FromLLVM for Operand<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    let is_instr = unsafe { !LLVMIsAInstruction(ptr).is_null() };
    if is_instr {
      Self::Instruction(Instruction::from_llvm(ptr))
    } else {
      let is_const = unsafe { !LLVMIsAConstant(ptr).is_null() };
      if is_const {
        Self::Constant(Constant::from_llvm(ptr))
      } else {
        Self::Metadata
      }
    }
  }
}
