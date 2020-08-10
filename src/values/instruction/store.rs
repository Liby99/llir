use llvm_sys::core::LLVMGetOperand;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Store instruction
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StoreInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> GetDebugMetadata<'ctx> for StoreInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for StoreInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for StoreInstruction<'ctx> {}

impl<'ctx> StoreInstruction<'ctx> {
  /// Get the location operand where the value is stored to
  pub fn location(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 1) })
  }

  /// Get the value operand
  pub fn value(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }
}

impl<'ctx> AsInstruction<'ctx> for StoreInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Store(*self)
  }
}

impl<'ctx> FromLLVMValue for StoreInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    StoreInstruction(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for StoreInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
