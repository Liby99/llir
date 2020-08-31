use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// MD Tuple Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MDTuple<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_as_operand_for_metadata!(MDTuple);

unsafe impl<'ctx> Send for MDTuple<'ctx> {}

unsafe impl<'ctx> Sync for MDTuple<'ctx> {}

impl_positional_value_ref!(MDTuple, 0);

impl_positional_from_llvm_value!(MDTuple);

impl<'ctx> AsMetadata<'ctx> for MDTuple<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::MDTuple(self.clone())
  }
}
