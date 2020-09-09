use llvm_sys::core::{LLVMGetConstOpcode, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Unary constant expression
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct UnaryConstExpr<'ctx>(UnaryOpcode, LLVMValueRef, PhantomData<&'ctx ()>);

impl_send_sync!(UnaryConstExpr);

impl<'ctx> UnaryConstExpr<'ctx> {
  /// Get the unary opcode
  pub fn opcode(&self) -> UnaryOpcode {
    self.0
  }

  /// Get the operand constant
  pub fn op0(&self) -> Constant<'ctx> {
    Constant::from_llvm(unsafe { LLVMGetOperand(self.1, 0) })
  }
}

impl<'ctx> GetType<'ctx> for UnaryConstExpr<'ctx> {}

impl<'ctx> ConstExprTrait<'ctx> for UnaryConstExpr<'ctx> {}

impl_op_from_llvm_value!(UnaryConstExpr, UnaryOpcode, LLVMGetConstOpcode);

impl_positional_value_ref!(UnaryConstExpr, 1);

impl<'ctx> AsConstExpr<'ctx> for UnaryConstExpr<'ctx> {
  fn as_const_expr(&self) -> ConstExpr<'ctx> {
    ConstExpr::Unary(self.clone())
  }
}

impl_const_expr_debug!(UnaryConstExpr);

impl_as_constant_and_as_operand_for_const_expr!(UnaryConstExpr);