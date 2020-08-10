use llvm_sys::core::{LLVMGetICmpPredicate, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// ICmp constant expression
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ICmpConstExpr<'ctx>(ICmpPredicate, LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> ICmpConstExpr<'ctx> {
  /// Get the icmp predicate
  pub fn predicate(&self) -> ICmpPredicate {
    self.0
  }

  /// Get the lhs operand
  pub fn op0(&self) -> Constant<'ctx> {
    Constant::from_llvm(unsafe { LLVMGetOperand(self.1, 0) })
  }

  /// Get the rhs operand
  pub fn op1(&self) -> Constant<'ctx> {
    Constant::from_llvm(unsafe { LLVMGetOperand(self.1, 1) })
  }
}

impl<'ctx> GetType<'ctx> for ICmpConstExpr<'ctx> {}

impl<'ctx> FromLLVMValue for ICmpConstExpr<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    let ll_pred = unsafe { LLVMGetICmpPredicate(ptr) };
    let pred = ICmpPredicate::from_llvm(ll_pred);
    ICmpConstExpr(pred, ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for ICmpConstExpr<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.1
  }
}

impl<'ctx> AsConstExpr<'ctx> for ICmpConstExpr<'ctx> {
  fn as_const_expr(&self) -> ConstExpr<'ctx> {
    ConstExpr::ICmp(self.clone())
  }
}
