use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// [Load instruction](https://llvm.org/docs/LangRef.html#load-instruction)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct LoadInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(LoadInstruction);

impl_as_operand_for_instr!(LoadInstruction);

unsafe impl<'ctx> Send for LoadInstruction<'ctx> {}

unsafe impl<'ctx> Sync for LoadInstruction<'ctx> {}

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
