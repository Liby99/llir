use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// DI Label Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DILabel<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_as_operand_for_metadata!(DILabel);

impl_send_sync!(DILabel);

impl_positional_value_ref!(DILabel, 0);

impl_positional_from_llvm_value!(DILabel);

impl<'ctx> AsMetadata<'ctx> for DILabel<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::DILabel(self.clone())
  }
}
