use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Local as Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LocalAsMetadata<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_as_operand_for_metadata!(LocalAsMetadata);

unsafe impl<'ctx> Send for LocalAsMetadata<'ctx> {}

unsafe impl<'ctx> Sync for LocalAsMetadata<'ctx> {}

impl_positional_value_ref!(LocalAsMetadata, 0);

impl_positional_from_llvm_value!(LocalAsMetadata);

impl<'ctx> AsMetadata<'ctx> for LocalAsMetadata<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::LocalAsMetadata(self.clone())
  }
}
