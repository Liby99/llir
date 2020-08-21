use llvm_sys::core::*;
use llvm_sys::debuginfo::*;
use llvm_sys::prelude::LLVMValueRef;

use crate::values::*;
use crate::{FromLLVMValue, ValueRef};

/// [Metadata](https://llvm.org/docs/LangRef.html#metadata)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Metadata<'ctx> {
  ConstantAsMetadata(ConstantAsMetadata<'ctx>),
  DIExpression(DIExpression<'ctx>),
  DILabel(DILabel<'ctx>),
  DILocalVariable(DILocalVariable<'ctx>),
  DILocation(DILocation<'ctx>),
  DistinctMDOperandPlaceholder(DistinctMDOperandPlaceholder<'ctx>),
  GenericDINode(GenericDINode<'ctx>),
  LocalAsMetadata(LocalAsMetadata<'ctx>),
  MDTuple(MDTuple<'ctx>),
  Other(GenericValue<'ctx>),
}

impl<'ctx> FromLLVMValue for Metadata<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    use LLVMMetadataKind::*;
    match unsafe { LLVMGetMetadataKind(LLVMValueAsMetadata(ptr)) } {
      LLVMConstantAsMetadataMetadataKind => Self::ConstantAsMetadata(ConstantAsMetadata::from_llvm(ptr)),
      LLVMDIExpressionMetadataKind => Self::DIExpression(DIExpression::from_llvm(ptr)),
      LLVMDILabelMetadataKind => Self::DILabel(DILabel::from_llvm(ptr)),
      LLVMDILocalVariableMetadataKind => Self::DILocalVariable(DILocalVariable::from_llvm(ptr)),
      LLVMDILocationMetadataKind => Self::DILocation(DILocation::from_llvm(ptr)),
      LLVMDistinctMDOperandPlaceholderMetadataKind => {
        Self::DistinctMDOperandPlaceholder(DistinctMDOperandPlaceholder::from_llvm(ptr))
      }
      LLVMGenericDINodeMetadataKind => Self::GenericDINode(GenericDINode::from_llvm(ptr)),
      LLVMLocalAsMetadataMetadataKind => Self::LocalAsMetadata(LocalAsMetadata::from_llvm(ptr)),
      LLVMMDTupleMetadataKind => Self::MDTuple(MDTuple::from_llvm(ptr)),
      _ => Self::Other(GenericValue::from_llvm(ptr)),
    }
  }
}

impl<'ctx> ValueRef for Metadata<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    match self {
      Self::ConstantAsMetadata(c) => c.value_ref(),
      Self::DIExpression(e) => e.value_ref(),
      Self::DILabel(l) => l.value_ref(),
      Self::DILocalVariable(lv) => lv.value_ref(),
      Self::DILocation(l) => l.value_ref(),
      Self::DistinctMDOperandPlaceholder(d) => d.value_ref(),
      Self::GenericDINode(g) => g.value_ref(),
      Self::LocalAsMetadata(lam) => lam.value_ref(),
      Self::MDTuple(t) => t.value_ref(),
      Self::Other(g) => g.value_ref(),
    }
  }
}

impl<'ctx> AsMetadata<'ctx> for Metadata<'ctx> {
  fn as_metadata(&self) -> Self {
    self.clone()
  }
}

impl_as_operand_for_metadata!(Metadata);
