use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GenericMDNode<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> GenericMDNode<'ctx> {

}

impl<'ctx> FromLLVMValue for GenericMDNode<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for GenericMDNode<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsMetadata<'ctx> for GenericMDNode<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::Generic(self.clone())
  }
}
