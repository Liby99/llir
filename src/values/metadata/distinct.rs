use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Distinct Metadata Operand Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DistinctMDNode<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> DistinctMDNode<'ctx> {}

impl<'ctx> FromLLVMValue for DistinctMDNode<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for DistinctMDNode<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsMetadata<'ctx> for DistinctMDNode<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::Distinct(self.clone())
  }
}
