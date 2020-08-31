use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Generic DI Node Metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GenericDINode<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_as_operand_for_metadata!(GenericDINode);

unsafe impl<'ctx> Send for GenericDINode<'ctx> {}

unsafe impl<'ctx> Sync for GenericDINode<'ctx> {}

impl_positional_value_ref!(GenericDINode, 0);

impl_positional_from_llvm_value!(GenericDINode);

impl<'ctx> AsMetadata<'ctx> for GenericDINode<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx> {
    Metadata::GenericDINode(self.clone())
  }
}
