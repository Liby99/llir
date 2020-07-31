use llvm_sys::core::LLVMGetOperand;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LoadInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> HasType for LoadInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for LoadInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for LoadInstruction<'ctx> {}

impl<'ctx> LoadInstruction<'ctx> {
  pub fn location(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }
}

impl<'ctx> AsInstruction<'ctx> for LoadInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Load(*self)
  }
}

impl<'ctx> FromLLVMValue for LoadInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    LoadInstruction(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for LoadInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
