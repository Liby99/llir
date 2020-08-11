use llvm_sys::core::*;
use llvm_sys::debuginfo::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// DILocation Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LocationMDNode<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for LocationMDNode<'ctx> {}

unsafe impl<'ctx> Sync for LocationMDNode<'ctx> {}

impl<'ctx> LocationMDNode<'ctx> {
  /// Get the column number
  pub fn col(&self) -> usize {
    unsafe { LLVMDILocationGetColumn(LLVMValueAsMetadata(self.0)) as usize }
  }

  /// Get the line number
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

impl<'ctx> AsMetadata<'ctx> for LocationMDNode<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::Location(self.clone())
  }
}
