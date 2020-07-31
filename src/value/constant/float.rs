use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::super::{ValueRef, FromLLVM};

#[derive(Copy, Clone)]
pub struct FloatConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> ValueRef for FloatConstant<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> FromLLVM for FloatConstant<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}