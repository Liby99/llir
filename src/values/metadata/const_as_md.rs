use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Constant as Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ConstantAsMetadata<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_as_operand_for_metadata!(ConstantAsMetadata);

unsafe impl<'ctx> Send for ConstantAsMetadata<'ctx> {}

unsafe impl<'ctx> Sync for ConstantAsMetadata<'ctx> {}

impl<'ctx> ConstantAsMetadata<'ctx> {}

impl<'ctx> FromLLVMValue for ConstantAsMetadata<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for ConstantAsMetadata<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsMetadata<'ctx> for ConstantAsMetadata<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::ConstantAsMetadata(self.clone())
  }
}
