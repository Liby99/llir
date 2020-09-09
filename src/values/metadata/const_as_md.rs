use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Constant as Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ConstantAsMetadata<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_as_operand_for_metadata!(ConstantAsMetadata);

impl_send_sync!(ConstantAsMetadata);

impl_positional_value_ref!(ConstantAsMetadata, 0);

impl_positional_from_llvm_value!(ConstantAsMetadata);

impl<'ctx> AsMetadata<'ctx> for ConstantAsMetadata<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::ConstantAsMetadata(self.clone())
  }
}
