use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Generic DI Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ConstantMDNode<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for ConstantMDNode<'ctx> {}

unsafe impl<'ctx> Sync for ConstantMDNode<'ctx> {}

impl<'ctx> ConstantMDNode<'ctx> {}

impl<'ctx> FromLLVMValue for ConstantMDNode<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for ConstantMDNode<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsMetadata<'ctx> for ConstantMDNode<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::Constant(self.clone())
  }
}
