use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// DI Expression Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DIExpression<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_as_operand_for_metadata!(DIExpression);

unsafe impl<'ctx> Send for DIExpression<'ctx> {}

unsafe impl<'ctx> Sync for DIExpression<'ctx> {}

impl<'ctx> DIExpression<'ctx> {}

impl<'ctx> FromLLVMValue for DIExpression<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for DIExpression<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsMetadata<'ctx> for DIExpression<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::DIExpression(self.clone())
  }
}
