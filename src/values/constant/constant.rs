use llvm_sys::core::LLVMGetValueKind;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMValueKind;
use std::marker::PhantomData;

use super::*;
use crate::values::{Function, Global};
use crate::{FromLLVMValue, ValueRef};

#[derive(Copy, Clone)]
pub enum Constant<'ctx> {
  Int(IntConstant<'ctx>),
  Float(FloatConstant<'ctx>),
  Null(NullConstant<'ctx>),
  Struct(StructConstant<'ctx>),
  Array(ArrayConstant<'ctx>),
  Vector(VectorConstant<'ctx>),
  Global(Global<'ctx>),
  Function(Function<'ctx>),
  ConstExpr(ConstExpr<'ctx>),
  Other(GenericConstant<'ctx>),
}

impl<'ctx> ValueRef for Constant<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    match self {
      Self::Int(ic) => ic.value_ref(),
      Self::Float(fc) => fc.value_ref(),
      Self::Null(nc) => nc.value_ref(),
      Self::Struct(sc) => sc.value_ref(),
      Self::Array(ac) => ac.value_ref(),
      Self::Vector(vc) => vc.value_ref(),
      Self::Function(fc) => fc.value_ref(),
      Self::Global(gc) => gc.value_ref(),
      Self::ConstExpr(cec) => cec.value_ref(),
      Self::Other(oc) => oc.value_ref(),
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
      LLVMConstantStructValueKind => Self::Struct(StructConstant::from_llvm(ptr)),
      LLVMConstantArrayValueKind => Self::Array(ArrayConstant::from_llvm(ptr)),
      LLVMConstantDataArrayValueKind => Self::Array(ArrayConstant::from_llvm(ptr)),
      LLVMConstantVectorValueKind => Self::Vector(VectorConstant::from_llvm(ptr)),
      LLVMConstantDataVectorValueKind => Self::Vector(VectorConstant::from_llvm(ptr)),
      LLVMConstantExprValueKind => Self::ConstExpr(ConstExpr::from_llvm(ptr)),
      LLVMFunctionValueKind => Self::Function(Function::from_llvm(ptr)),
      LLVMGlobalAliasValueKind => Self::Global(Global::from_llvm(ptr)),
      _ => Self::Other(GenericConstant::from_llvm(ptr)),
    }
  }
}

#[derive(Copy, Clone)]
pub struct GenericConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> FromLLVMValue for GenericConstant<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for GenericConstant<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
