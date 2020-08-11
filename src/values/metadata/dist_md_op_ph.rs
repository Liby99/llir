use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Distinct Metadata Operand Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DistinctMDOperandPlaceholder<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for DistinctMDOperandPlaceholder<'ctx> {}

unsafe impl<'ctx> Sync for DistinctMDOperandPlaceholder<'ctx> {}

impl<'ctx> DistinctMDOperandPlaceholder<'ctx> {}

impl<'ctx> FromLLVMValue for DistinctMDOperandPlaceholder<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for DistinctMDOperandPlaceholder<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsMetadata<'ctx> for DistinctMDOperandPlaceholder<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::DistinctMDOperandPlaceholder(self.clone())
  }
}
