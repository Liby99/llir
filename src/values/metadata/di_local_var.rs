use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// DI Local Variable Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DILocalVariable<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_as_operand_for_metadata!(DILocalVariable);

unsafe impl<'ctx> Send for DILocalVariable<'ctx> {}

unsafe impl<'ctx> Sync for DILocalVariable<'ctx> {}

impl<'ctx> DILocalVariable<'ctx> {}

impl<'ctx> FromLLVMValue for DILocalVariable<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for DILocalVariable<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsMetadata<'ctx> for DILocalVariable<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::DILocalVariable(self.clone())
  }
}
