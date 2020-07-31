use llvm_sys::core::{LLVMGetNumOperands, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ReturnInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> InstructionDebugLoc for ReturnInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for ReturnInstruction<'ctx> {}

impl<'ctx> ReturnInstruction<'ctx> {
  pub fn has_op(&self) -> bool {
    unsafe { LLVMGetNumOperands(self.0) != 0 }
  }

  pub fn op(&self) -> Option<Operand<'ctx>> {
    if self.has_op() {
      Some(Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) }))
    } else {
      None
    }
  }
}

impl<'ctx> AsInstruction<'ctx> for ReturnInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Return(*self)
  }
}

impl<'ctx> FromLLVMValue for ReturnInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for ReturnInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
