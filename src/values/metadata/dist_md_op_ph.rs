use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Distinct Metadata Operand Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DistinctMDOperandPlaceholder<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_as_operand_for_metadata!(DistinctMDOperandPlaceholder);

impl_send_sync!(DistinctMDOperandPlaceholder);

impl_positional_value_ref!(DistinctMDOperandPlaceholder, 0);

impl_positional_from_llvm_value!(DistinctMDOperandPlaceholder);

impl<'ctx> AsMetadata<'ctx> for DistinctMDOperandPlaceholder<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::DistinctMDOperandPlaceholder(self.clone())
  }
}
