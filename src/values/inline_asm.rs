use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::*;
use crate::types::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct InlineAsm<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> InlineAsm<'ctx> {
  pub fn function_type(&self) -> FunctionType<'ctx> {
    FunctionType::from_llvm(unsafe { LLVMGetElementType(LLVMTypeOf(self.0)) })
  }
}

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
