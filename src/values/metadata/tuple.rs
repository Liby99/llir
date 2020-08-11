use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Generic DI Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TupleMDNode<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for TupleMDNode<'ctx> {}

unsafe impl<'ctx> Sync for TupleMDNode<'ctx> {}

impl<'ctx> TupleMDNode<'ctx> {}

impl<'ctx> FromLLVMValue for TupleMDNode<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for TupleMDNode<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsMetadata<'ctx> for TupleMDNode<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::Tuple(self.clone())
  }
}
