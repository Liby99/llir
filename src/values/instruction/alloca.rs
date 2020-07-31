use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::{FromLLVMValue, ValueRef};

#[derive(Debug, Copy, Clone)]
pub struct AllocaInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> AllocaInstruction<'ctx> {}

impl<'ctx> FromLLVMValue for AllocaInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    AllocaInstruction(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for AllocaInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
