use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Generic DI Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LabelMDNode<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for LabelMDNode<'ctx> {}

unsafe impl<'ctx> Sync for LabelMDNode<'ctx> {}

impl<'ctx> LabelMDNode<'ctx> {}

impl<'ctx> FromLLVMValue for LabelMDNode<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for LabelMDNode<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsMetadata<'ctx> for LabelMDNode<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::Label(self.clone())
  }
}
