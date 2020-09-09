use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::utils::*;
use crate::*;

/// A placeholder value; used when the value kind is not supported yet
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GenericValue<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_send_sync!(GenericValue);

impl<'ctx> GenericValue<'ctx> {
  pub(crate) fn new(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }

  /// Get the name of the generic value. It might or might not have one
  pub fn name(&self) -> Option<String> {
    let s = string_of_value(self.0);
    if s.len() == 0 {
      None
    } else {
      Some(s)
    }
  }
}

impl_positional_value_ref!(GenericValue, 0);

impl_positional_from_llvm_value!(GenericValue);
