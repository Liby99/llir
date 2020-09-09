use llvm_sys::core::{LLVMGetFCmpPredicate, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// FCmp constant expression
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct FCmpConstExpr<'ctx>(FCmpPredicate, LLVMValueRef, PhantomData<&'ctx ()>);

impl_send_sync!(FCmpConstExpr);

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

impl<'ctx> ConstExprTrait<'ctx> for FCmpConstExpr<'ctx> {}

impl_cmp_from_llvm_value!(FCmpConstExpr, FCmpPredicate, LLVMGetFCmpPredicate);

impl_positional_value_ref!(FCmpConstExpr, 1);

impl<'ctx> AsConstExpr<'ctx> for FCmpConstExpr<'ctx> {
  fn as_const_expr(&self) -> ConstExpr<'ctx> {
    ConstExpr::FCmp(self.clone())
  }
}

impl_const_expr_debug!(FCmpConstExpr);

impl_as_constant_and_as_operand_for_const_expr!(FCmpConstExpr);