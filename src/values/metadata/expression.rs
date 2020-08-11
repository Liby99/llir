use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Generic DI Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExpressionMDNode<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for ExpressionMDNode<'ctx> {}

unsafe impl<'ctx> Sync for ExpressionMDNode<'ctx> {}

impl<'ctx> ExpressionMDNode<'ctx> {}

impl<'ctx> FromLLVMValue for ExpressionMDNode<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for ExpressionMDNode<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsMetadata<'ctx> for ExpressionMDNode<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::Expression(self.clone())
  }
}
