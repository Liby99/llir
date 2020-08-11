use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// DI Label Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DILabel<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for DILabel<'ctx> {}

unsafe impl<'ctx> Sync for DILabel<'ctx> {}

impl<'ctx> DILabel<'ctx> {}

impl<'ctx> FromLLVMValue for DILabel<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for DILabel<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsMetadata<'ctx> for DILabel<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::DILabel(self.clone())
  }
}
