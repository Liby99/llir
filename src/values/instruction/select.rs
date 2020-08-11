use llvm_sys::core::LLVMGetOperand;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Alloca instruction
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SelectInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for SelectInstruction<'ctx> {}

unsafe impl<'ctx> Sync for SelectInstruction<'ctx> {}

impl<'ctx> GetType<'ctx> for SelectInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for SelectInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for SelectInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for SelectInstruction<'ctx> {}

impl<'ctx> AsInstruction<'ctx> for SelectInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Select(*self)
  }
}

impl<'ctx> SelectInstruction<'ctx> {
  pub fn condition(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  pub fn true_value(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 1) })
  }

  pub fn false_value(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 2) })
  }
}

impl<'ctx> FromLLVMValue for SelectInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for SelectInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
