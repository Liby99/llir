use llvm_sys::core::{LLVMGetICmpPredicate, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// ICmp constant expression
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ICmpConstExpr<'ctx>(ICmpPredicate, LLVMValueRef, PhantomData<&'ctx ()>);

impl_send_sync!(ICmpConstExpr);

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

impl<'ctx> ConstExprTrait<'ctx> for ICmpConstExpr<'ctx> {}

impl_cmp_from_llvm_value!(ICmpConstExpr, ICmpPredicate, LLVMGetICmpPredicate);

impl_positional_value_ref!(ICmpConstExpr, 1);

impl<'ctx> AsConstExpr<'ctx> for ICmpConstExpr<'ctx> {
  fn as_const_expr(&self) -> ConstExpr<'ctx> {
    ConstExpr::ICmp(self.clone())
  }
}

impl_const_expr_debug!(ICmpConstExpr);

impl_as_constant_and_as_operand_for_const_expr!(ICmpConstExpr);