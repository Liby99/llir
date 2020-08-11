use llvm_sys::core::{LLVMGetFCmpPredicate, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// FCmp constant expression
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FCmpConstExpr<'ctx>(FCmpPredicate, LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for FCmpConstExpr<'ctx> {}

unsafe impl<'ctx> Sync for FCmpConstExpr<'ctx> {}

impl<'ctx> FCmpConstExpr<'ctx> {
  /// Get the fcmp predicate
  pub fn predicate(&self) -> FCmpPredicate {
    self.0
  }

  /// Get the lhs operand in constant
  pub fn op0(&self) -> Constant<'ctx> {
    Constant::from_llvm(unsafe { LLVMGetOperand(self.1, 0) })
  }

  /// Get the rhs operand in constant
  pub fn op1(&self) -> Constant<'ctx> {
    Constant::from_llvm(unsafe { LLVMGetOperand(self.1, 1) })
  }
}

impl<'ctx> GetType<'ctx> for FCmpConstExpr<'ctx> {}

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

impl<'ctx> AsConstExpr<'ctx> for FCmpConstExpr<'ctx> {
  fn as_const_expr(&self) -> ConstExpr<'ctx> {
    ConstExpr::FCmp(self.clone())
  }
}
