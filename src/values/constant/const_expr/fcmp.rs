use llvm_sys::core::{LLVMGetFCmpPredicate, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::{Constant, FCmpPredicate};
use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FCmpConstExpr<'ctx>(FCmpPredicate, LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> FCmpConstExpr<'ctx> {
  pub fn predicate(&self) -> FCmpPredicate {
    self.0
  }

  pub fn op0(&self) -> Constant<'ctx> {
    Constant::from_llvm(unsafe { LLVMGetOperand(self.1, 0) })
  }

  pub fn op1(&self) -> Constant<'ctx> {
    Constant::from_llvm(unsafe { LLVMGetOperand(self.1, 1) })
  }
}

impl<'ctx> FromLLVMValue for FCmpConstExpr<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    let ll_pred = unsafe { LLVMGetFCmpPredicate(ptr) };
    let pred = FCmpPredicate::from_llvm(ll_pred);
    FCmpConstExpr(pred, ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for FCmpConstExpr<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.1
  }
}