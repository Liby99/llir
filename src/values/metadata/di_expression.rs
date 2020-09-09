use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// DI Expression Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DIExpression<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_as_operand_for_metadata!(DIExpression);

impl_send_sync!(DIExpression);

impl_positional_value_ref!(DIExpression, 0);

impl_positional_from_llvm_value!(DIExpression);

impl<'ctx> AsMetadata<'ctx> for DIExpression<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::DIExpression(self.clone())
  }
}
