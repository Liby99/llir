use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::{ValueRef, FromLLVMValue};

#[derive(Copy, Clone)]
pub struct NullConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> ValueRef for NullConstant<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> FromLLVMValue for NullConstant<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}