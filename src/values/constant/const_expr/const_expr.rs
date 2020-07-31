use llvm_sys::core::LLVMGetConstOpcode;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMOpcode;
use std::marker::PhantomData;

use super::*;
use crate::values::*;
use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConstExpr<'ctx> {
  Binary(BinaryConstExpr<'ctx>),
  Unary(UnaryConstExpr<'ctx>),
  ICmp(ICmpConstExpr<'ctx>),
  FCmp(FCmpConstExpr<'ctx>),
  GetElementPtr(GetElementPtrConstExpr<'ctx>),
  Other(GenericConstExpr<'ctx>),
}

impl<'ctx> FromLLVMValue for ConstExpr<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    use LLVMOpcode::*;
    match unsafe { LLVMGetConstOpcode(ptr) } {
      LLVMGetElementPtr => Self::GetElementPtr(GetElementPtrConstExpr::from_llvm(ptr)),
      LLVMICmp => Self::ICmp(ICmpConstExpr::from_llvm(ptr)),
      LLVMFCmp => Self::FCmp(FCmpConstExpr::from_llvm(ptr)),
      o if BinaryOpcode::from_llvm(o).is_some() => Self::Binary(BinaryConstExpr::from_llvm(ptr)),
      o if UnaryOpcode::from_llvm(o).is_some() => Self::Unary(UnaryConstExpr::from_llvm(ptr)),
      _ => Self::Other(GenericConstExpr::from_llvm(ptr)),
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GenericConstExpr<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> FromLLVMValue for GenericConstExpr<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for GenericConstExpr<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
