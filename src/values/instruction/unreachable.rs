use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UnreachableInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> InstructionDebugLoc for UnreachableInstruction<'ctx> {}

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
