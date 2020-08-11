use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Generic DI Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LocalAsMetadataMDNode<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for LocalAsMetadataMDNode<'ctx> {}

unsafe impl<'ctx> Sync for LocalAsMetadataMDNode<'ctx> {}

impl<'ctx> LocalAsMetadataMDNode<'ctx> {}

impl<'ctx> FromLLVMValue for LocalAsMetadataMDNode<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for LocalAsMetadataMDNode<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsMetadata<'ctx> for LocalAsMetadataMDNode<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::LocalAsMetadata(self.clone())
  }
}
