use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// [Unreachable instruction](https://llvm.org/docs/LangRef.html#unreachable-instruction)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct UnreachableInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for UnreachableInstruction<'ctx> {}

unsafe impl<'ctx> Sync for UnreachableInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for UnreachableInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for UnreachableInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for UnreachableInstruction<'ctx> {}

impl<'ctx> AsInstruction<'ctx> for UnreachableInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Unreachable(*self)
  }
}

impl<'ctx> FromLLVMValue for UnreachableInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    UnreachableInstruction(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for UnreachableInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
