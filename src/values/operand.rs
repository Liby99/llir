use llvm_sys::core::{LLVMIsAConstant, LLVMIsAInstruction, LLVMIsAMDNode};
use llvm_sys::prelude::LLVMValueRef;

use crate::values::*;
use crate::*;

#[derive(Copy, Clone)]
pub enum Operand<'ctx> {
  Instruction(Instruction<'ctx>),
  Constant(Constant<'ctx>),
  Metadata(Metadata<'ctx>), // TODO
}

impl<'ctx> HasType for Operand<'ctx> {}

impl<'ctx> FromLLVMValue for Operand<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    let is_instr = unsafe { !LLVMIsAInstruction(ptr).is_null() };
    if is_instr {
      Self::Instruction(Instruction::from_llvm(ptr))
    } else {
      let is_const = unsafe { !LLVMIsAConstant(ptr).is_null() };
      if is_const {
        Self::Constant(Constant::from_llvm(ptr))
      } else {
        let is_mdnode = unsafe { !LLVMIsAMDNode(ptr).is_null() };
        if is_mdnode {
          Self::Metadata(Metadata::from_llvm(ptr))
        } else {
          panic!("Unsupported value {:?}", ptr);
        }
      }
    }
  }
}

impl<'ctx> ValueRef for Operand<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    match self {
      Operand::Instruction(instr) => instr.value_ref(),
      Operand::Constant(constant) => constant.value_ref(),
      Operand::Metadata(metadata) => metadata.value_ref(),
    }
  }
}
