use llvm_sys::core::*;
use llvm_sys::debuginfo::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LocationMDNode<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> LocationMDNode<'ctx> {
  pub fn col(&self) -> usize {
    unsafe { LLVMDILocationGetColumn(LLVMValueAsMetadata(self.0)) as usize }
  }

  pub fn line(&self) -> usize {
    unsafe { LLVMDILocationGetLine(LLVMValueAsMetadata(self.0)) as usize }
  }
}

impl<'ctx> FromLLVMValue for LocationMDNode<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for LocationMDNode<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
