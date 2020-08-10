use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::debuginfo::*;

use crate::values::*;
use crate::{FromLLVMValue, ValueRef};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Metadata<'ctx> {
  Distinct(DistinctMDNode<'ctx>),
  Location(LocationMDNode<'ctx>),
  Other(GenericValue<'ctx>),
}

impl<'ctx> FromLLVMValue for Metadata<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    use LLVMMetadataKind::*;
    match unsafe { LLVMGetMetadataKind(LLVMValueAsMetadata(ptr)) } {
      LLVMDistinctMDOperandPlaceholderMetadataKind => Self::Distinct(DistinctMDNode::from_llvm(ptr)),
      LLVMDILocationMetadataKind => Self::Location(LocationMDNode::from_llvm(ptr)),
      x => {
        println!("{:?}", x);
        Self::Other(GenericValue::from_llvm(ptr))
      }
    }
  }
}

impl<'ctx> ValueRef for Metadata<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    match self {
      Self::Distinct(d) => d.value_ref(),
      Self::Location(l) => l.value_ref(),
      Self::Other(g) => g.value_ref(),
    }
  }
}
