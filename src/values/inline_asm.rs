use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct InlineAsm<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> FromLLVMValue for InlineAsm<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for InlineAsm<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
