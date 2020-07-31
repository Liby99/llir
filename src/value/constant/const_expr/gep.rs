use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::{FromLLVMValue, ValueRef};

#[derive(Copy, Clone)]
pub struct GetElementPtrConstExpr<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> FromLLVMValue for GetElementPtrConstExpr<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for GetElementPtrConstExpr<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}