use llvm_sys::core::LLVMGetOperand;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Load instruction
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LoadInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> GetType<'ctx> for LoadInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for LoadInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for LoadInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for LoadInstruction<'ctx> {}

impl<'ctx> LoadInstruction<'ctx> {
  /// Get the location operand which the value is load from
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
