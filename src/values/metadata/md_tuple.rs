use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// MD Tuple Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MDTuple<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_as_operand_for_metadata!(MDTuple);

unsafe impl<'ctx> Send for MDTuple<'ctx> {}

unsafe impl<'ctx> Sync for MDTuple<'ctx> {}

impl<'ctx> MDTuple<'ctx> {}

impl<'ctx> FromLLVMValue for MDTuple<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for MDTuple<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsMetadata<'ctx> for MDTuple<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::MDTuple(self.clone())
  }
}
