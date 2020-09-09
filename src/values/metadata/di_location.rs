use llvm_sys::core::*;
use llvm_sys::debuginfo::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// DI Location Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DILocation<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_as_operand_for_metadata!(DILocation);

impl_send_sync!(DILocation);

impl<'ctx> DILocation<'ctx> {
  /// Get the column number
  pub fn col(&self) -> usize {
    unsafe { LLVMDILocationGetColumn(LLVMValueAsMetadata(self.0)) as usize }
  }

  /// Get the line number
  pub fn line(&self) -> usize {
    unsafe { LLVMDILocationGetLine(LLVMValueAsMetadata(self.0)) as usize }
  }
}

impl_positional_value_ref!(DILocation, 0);

impl_positional_from_llvm_value!(DILocation);

impl<'ctx> AsMetadata<'ctx> for DILocation<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::DILocation(self.clone())
  }
}
