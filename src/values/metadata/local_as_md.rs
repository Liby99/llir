use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Local as Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LocalAsMetadata<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for LocalAsMetadata<'ctx> {}

unsafe impl<'ctx> Sync for LocalAsMetadata<'ctx> {}

impl<'ctx> LocalAsMetadata<'ctx> {}

impl<'ctx> FromLLVMValue for LocalAsMetadata<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for LocalAsMetadata<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsMetadata<'ctx> for LocalAsMetadata<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::LocalAsMetadata(self.clone())
  }
}
