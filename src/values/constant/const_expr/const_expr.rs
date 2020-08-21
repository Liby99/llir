use llvm_sys::core::LLVMGetConstOpcode;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMOpcode;

use super::*;
use crate::values::*;
use crate::*;

/// [Constant expression](https://llvm.org/docs/LangRef.html#constant-expressions)
///
/// Super class for all constant expressions
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConstExpr<'ctx> {
  Binary(BinaryConstExpr<'ctx>),
  Unary(UnaryConstExpr<'ctx>),
  ICmp(ICmpConstExpr<'ctx>),
  FCmp(FCmpConstExpr<'ctx>),
  GetElementPtr(GetElementPtrConstExpr<'ctx>),
  Other(GenericValue<'ctx>),
}

impl<'ctx> GetType<'ctx> for ConstExpr<'ctx> {}

impl<'ctx> ConstExprTrait<'ctx> for ConstExpr<'ctx> {}

impl<'ctx> FromLLVMValue for ConstExpr<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    use LLVMOpcode::*;
    match unsafe { LLVMGetConstOpcode(ptr) } {
      LLVMGetElementPtr => Self::GetElementPtr(GetElementPtrConstExpr::from_llvm(ptr)),
      LLVMICmp => Self::ICmp(ICmpConstExpr::from_llvm(ptr)),
      LLVMFCmp => Self::FCmp(FCmpConstExpr::from_llvm(ptr)),
      o if BinaryOpcode::from_llvm(o).is_some() => Self::Binary(BinaryConstExpr::from_llvm(ptr)),
      o if UnaryOpcode::from_llvm(o).is_some() => Self::Unary(UnaryConstExpr::from_llvm(ptr)),
      _ => Self::Other(GenericValue::from_llvm(ptr)),
    }
  }
}

impl<'ctx> ValueRef for ConstExpr<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    match self {
      Self::Binary(b) => b.value_ref(),
      Self::Unary(u) => u.value_ref(),
      Self::GetElementPtr(g) => g.value_ref(),
      Self::ICmp(i) => i.value_ref(),
      Self::FCmp(f) => f.value_ref(),
      Self::Other(g) => g.value_ref(),
    }
  }
}

impl<'ctx> AsConstExpr<'ctx> for ConstExpr<'ctx> {
  fn as_const_expr(&self) -> ConstExpr<'ctx> {
    self.clone()
  }
}

impl_as_constant_and_as_operand_for_const_expr!(ConstExpr);