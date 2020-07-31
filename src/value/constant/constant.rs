use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::core::{LLVMGetValueKind};
use llvm_sys::LLVMValueKind;

use crate::{FromLLVMValue, ValueRef};
use super::*;

#[derive(Copy, Clone)]
pub enum Constant<'ctx> {
  Int(IntConstant<'ctx>),
  Float(FloatConstant<'ctx>),
  Null(NullConstant<'ctx>),
}

impl<'ctx> ValueRef for Constant<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    match self {
      Self::Int(ic) => ic.value_ref(),
      Self::Float(fc) => fc.value_ref(),
      Self::Null(nc) => nc.value_ref(),
    }
  }
}

impl<'ctx> FromLLVMValue for Constant<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    use LLVMValueKind::*;
    match unsafe { LLVMGetValueKind(ptr) } {
      LLVMConstantIntValueKind => Self::Int(IntConstant::from_llvm(ptr)),
      LLVMConstantFPValueKind => Self::Float(FloatConstant::from_llvm(ptr)),
      LLVMConstantPointerNullValueKind => Self::Null(NullConstant::from_llvm(ptr)),
      _ => panic!("Unsupported value kind")
    }
  }
}
