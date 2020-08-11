use llvm_sys::core::*;
use llvm_sys::debuginfo::*;
use llvm_sys::prelude::LLVMValueRef;

use crate::values::*;
use crate::{FromLLVMValue, ValueRef};

/// [Metadata](https://llvm.org/docs/LangRef.html#metadata)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Metadata<'ctx> {
  Constant(ConstantMDNode<'ctx>),
  Distinct(DistinctMDNode<'ctx>),
  Expression(ExpressionMDNode<'ctx>),
  Label(LabelMDNode<'ctx>),
  LocalAsMetadata(LocalAsMetadataMDNode<'ctx>),
  LocalVariable(LocalVariableMDNode<'ctx>),
  Location(LocationMDNode<'ctx>),
  Generic(GenericMDNode<'ctx>),
  Tuple(TupleMDNode<'ctx>),
  Other(GenericValue<'ctx>),
}

impl<'ctx> FromLLVMValue for Metadata<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    use LLVMMetadataKind::*;
    match unsafe { LLVMGetMetadataKind(LLVMValueAsMetadata(ptr)) } {
      LLVMConstantAsMetadataMetadataKind => Self::Constant(ConstantMDNode::from_llvm(ptr)),
      LLVMDistinctMDOperandPlaceholderMetadataKind => Self::Distinct(DistinctMDNode::from_llvm(ptr)),
      LLVMDIExpressionMetadataKind => Self::Expression(ExpressionMDNode::from_llvm(ptr)),
      LLVMDILabelMetadataKind => Self::Label(LabelMDNode::from_llvm(ptr)),
      LLVMDILocationMetadataKind => Self::Location(LocationMDNode::from_llvm(ptr)),
      LLVMDILocalVariableMetadataKind => Self::LocalVariable(LocalVariableMDNode::from_llvm(ptr)),
      LLVMLocalAsMetadataMetadataKind => Self::LocalAsMetadata(LocalAsMetadataMDNode::from_llvm(ptr)),
      LLVMMDTupleMetadataKind => Self::Tuple(TupleMDNode::from_llvm(ptr)),
      LLVMGenericDINodeMetadataKind => Self::Generic(GenericMDNode::from_llvm(ptr)),
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
      Self::Constant(c) => c.value_ref(),
      Self::Distinct(d) => d.value_ref(),
      Self::Expression(e) => e.value_ref(),
      Self::Label(l) => l.value_ref(),
      Self::LocalAsMetadata(lam) => lam.value_ref(),
      Self::LocalVariable(lv) => lv.value_ref(),
      Self::Location(l) => l.value_ref(),
      Self::Generic(g) => g.value_ref(),
      Self::Tuple(t) => t.value_ref(),
      Self::Other(g) => g.value_ref(),
    }
  }
}

impl<'ctx> AsMetadata<'ctx> for Metadata<'ctx> {
  fn as_metadata(&self) -> Self {
    self.clone()
  }
}
