use llvm_sys::core::{LLVMGetConstOpcode, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Binary constant expression
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct BinaryConstExpr<'ctx>(BinaryOpcode, LLVMValueRef, PhantomData<&'ctx ()>);

impl_send_sync!(BinaryConstExpr);

impl<'ctx> BinaryConstExpr<'ctx> {
  /// Get the opcode
  pub fn opcode(&self) -> BinaryOpcode {
    self.0
  }

  pub fn op0(&self) -> Constant<'ctx> {
    Constant::from_llvm(unsafe { LLVMGetOperand(self.1, 0) })
  }

  pub fn op1(&self) -> Constant<'ctx> {
    Constant::from_llvm(unsafe { LLVMGetOperand(self.1, 1) })
  }
}

impl<'ctx> GetType<'ctx> for BinaryConstExpr<'ctx> {}

impl<'ctx> ConstExprTrait<'ctx> for BinaryConstExpr<'ctx> {}

impl_positional_value_ref!(BinaryConstExpr, 1);

impl_op_from_llvm_value!(BinaryConstExpr, BinaryOpcode, LLVMGetConstOpcode);

impl<'ctx> AsConstExpr<'ctx> for BinaryConstExpr<'ctx> {
  fn as_const_expr(&self) -> ConstExpr<'ctx> {
    ConstExpr::Binary(self.clone())
  }
}

impl_const_expr_debug!(BinaryConstExpr);

impl_as_constant_and_as_operand_for_const_expr!(BinaryConstExpr);