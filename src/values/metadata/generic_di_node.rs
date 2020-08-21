use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Generic DI Node Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GenericDINode<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_as_operand_for_metadata!(GenericDINode);

unsafe impl<'ctx> Send for GenericDINode<'ctx> {}

unsafe impl<'ctx> Sync for GenericDINode<'ctx> {}

impl<'ctx> GenericDINode<'ctx> {}

impl<'ctx> FromLLVMValue for GenericDINode<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for GenericDINode<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsMetadata<'ctx> for GenericDINode<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::GenericDINode(self.clone())
  }
}
