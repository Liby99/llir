use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Generic DI Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LocalVariableMDNode<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for LocalVariableMDNode<'ctx> {}

unsafe impl<'ctx> Sync for LocalVariableMDNode<'ctx> {}

impl<'ctx> LocalVariableMDNode<'ctx> {}

impl<'ctx> FromLLVMValue for LocalVariableMDNode<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for LocalVariableMDNode<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsMetadata<'ctx> for LocalVariableMDNode<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::LocalVariable(self.clone())
  }
}
