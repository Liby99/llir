use llvm_sys::core::LLVMGetValueKind;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMValueKind;

use super::*;
use crate::values::*;
use crate::*;

/// [Constants](https://llvm.org/docs/LangRef.html#constants)
///
/// Int, Float, and Null are [Simple Constants](https://llvm.org/docs/LangRef.html#simple-constants)
///
/// Struct, Array, and Vector are [Complex Constants](https://llvm.org/docs/LangRef.html#complex-constants)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Constant<'ctx> {
  Int(IntConstant<'ctx>),
  Float(FloatConstant<'ctx>),
  Null(NullConstant<'ctx>),
  Struct(StructConstant<'ctx>),
  Array(ArrayConstant<'ctx>),
  Vector(VectorConstant<'ctx>),
  BlockAddress(BlockAddress<'ctx>),
  Undef(Undef<'ctx>),
  Global(Global<'ctx>),
  Function(Function<'ctx>),
  ConstExpr(ConstExpr<'ctx>),
  Other(GenericValue<'ctx>),
}

impl<'ctx> Constant<'ctx> {
  /// Turn constant into an operand
  pub fn as_operand(&self) -> Operand<'ctx> {
    Operand::Constant(*self)
  }
}

impl<'ctx> GetType<'ctx> for Constant<'ctx> {}

impl<'ctx> ValueRef for Constant<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    match self {
      Self::Int(ic) => ic.value_ref(),
      Self::Float(fc) => fc.value_ref(),
      Self::Null(nc) => nc.value_ref(),
      Self::Struct(sc) => sc.value_ref(),
      Self::Array(ac) => ac.value_ref(),
      Self::Vector(vc) => vc.value_ref(),
      Self::BlockAddress(ba) => ba.value_ref(),
      Self::Undef(ud) => ud.value_ref(),
      Self::Global(gc) => gc.value_ref(),
      Self::Function(fc) => fc.value_ref(),
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
      LLVMBlockAddressValueKind => Self::BlockAddress(BlockAddress::from_llvm(ptr)),
      LLVMUndefValueValueKind => Self::Undef(Undef::from_llvm(ptr)),
      LLVMGlobalIFuncValueKind | LLVMFunctionValueKind => Self::Function(Function::from_llvm(ptr)),
      LLVMGlobalAliasValueKind | LLVMGlobalVariableValueKind => Self::Global(Global::from_llvm(ptr)),
      LLVMConstantExprValueKind => Self::ConstExpr(ConstExpr::from_llvm(ptr)),
      _ => Self::Other(GenericValue::from_llvm(ptr)),
    }
  }
}

impl<'ctx> AsConstant<'ctx> for Constant<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    self.clone()
  }
}

impl_as_operand_for_constant!(Constant);
