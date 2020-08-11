use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::utils::*;
use crate::*;

/// A placeholder value; used when the value kind is not supported yet
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GenericValue<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for GenericValue<'ctx> {}

unsafe impl<'ctx> Sync for GenericValue<'ctx> {}

impl<'ctx> GenericValue<'ctx> {
  pub(crate) fn new(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }

  /// Get the name of the generic value. It might or might not have one
  pub fn name(&self) -> Option<String> {
    let s = string_of_value(self.0);
    if s.len() == 0 {
      None
    } else {
      Some(s)
    }
  }
}

impl<'ctx> FromLLVMValue for GenericValue<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for GenericValue<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}