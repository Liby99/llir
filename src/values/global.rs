use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::utils::string_of_value;
use crate::{FromLLVMValue, ValueRef};

#[derive(Copy, Clone)]
pub struct Global<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> Global<'ctx> {
  pub fn name(&self) -> String {
    string_of_value(self.0)
  }
}

impl<'ctx> FromLLVMValue for Global<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef  for Global<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}