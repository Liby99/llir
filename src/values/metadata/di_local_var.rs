use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// DI Local Variable Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DILocalVariable<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_as_operand_for_metadata!(DILocalVariable);

impl_send_sync!(DILocalVariable);

impl_positional_value_ref!(DILocalVariable, 0);

impl_positional_from_llvm_value!(DILocalVariable);

impl<'ctx> AsMetadata<'ctx> for DILocalVariable<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::DILocalVariable(self.clone())
  }
}
