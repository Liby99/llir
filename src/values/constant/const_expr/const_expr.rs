use llvm_sys::core::LLVMGetConstOpcode;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMOpcode;

use super::*;
use crate::values::*;
use crate::{FromLLVMValue, ValueRef};

#[derive(Copy, Clone)]
pub enum ConstExpr<'ctx> {
  Binary(BinaryConstExpr<'ctx>),
  Unary(UnaryConstExpr<'ctx>),
  GetElementPtr(GetElementPtrConstExpr<'ctx>),
}

impl<'ctx> FromLLVMValue for ConstExpr<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    use LLVMOpcode::*;
    match unsafe { LLVMGetConstOpcode(ptr) } {
      LLVMGetElementPtr => Self::GetElementPtr(GetElementPtrConstExpr::from_llvm(ptr)),
      o if BinaryOpcode::from_llvm(o).is_some() => Self::Binary(BinaryConstExpr::from_llvm(ptr)),
      o if UnaryOpcode::from_llvm(o).is_some() => Self::Unary(UnaryConstExpr::from_llvm(ptr)),
      x => panic!("Not supported const opcode {:?}", x),
    }
  }
}

impl<'ctx> ValueRef for ConstExpr<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    match self {
      Self::Binary(b) => b.value_ref(),
      Self::Unary(u) => u.value_ref(),
      Self::GetElementPtr(g) => g.value_ref(),
    }
  }
}
