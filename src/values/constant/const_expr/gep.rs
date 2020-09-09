use llvm_sys::core::{LLVMGetNumOperands, LLVMGetOperand, LLVMGetValueKind};
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMValueKind;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Get element pointer constant expression
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct GetElementPtrConstExpr<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_send_sync!(GetElementPtrConstExpr);

impl<'ctx> GetElementPtrConstExpr<'ctx> {
  /// Get the base location constant
  pub fn location(&self) -> Constant<'ctx> {
    Constant::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  /// Get the number of indices used to get the element pointer
  pub fn num_indices(&self) -> usize {
    (unsafe { LLVMGetNumOperands(self.0) as usize }) - 1
  }

  /// Get the indices in Constant Vector format
  pub fn indices(&self) -> Vec<Constant<'ctx>> {
    (0..self.num_indices() as u32)
      .map(|i| Constant::from_llvm(unsafe { LLVMGetOperand(self.0, i + 1) }))
      .collect()
  }

  /// Get the indices in Integer Constant Vector format, since the constants used
  /// in a GEP Const Expr can only be integer constant
  pub fn int_indices(&self) -> Vec<IntConstant<'ctx>> {
    (0..self.num_indices() as u32)
      .map(|i| {
        let operand = unsafe { LLVMGetOperand(self.0, i + 1) };
        assert_eq!(
          unsafe { LLVMGetValueKind(operand) },
          LLVMValueKind::LLVMConstantIntValueKind
        );
        IntConstant::from_llvm(operand)
      })
      .collect()
  }
}

impl<'ctx> GetType<'ctx> for GetElementPtrConstExpr<'ctx> {}

impl<'ctx> ConstExprTrait<'ctx> for GetElementPtrConstExpr<'ctx> {}

impl_positional_value_ref!(GetElementPtrConstExpr, 0);

impl_positional_from_llvm_value!(GetElementPtrConstExpr);

impl<'ctx> AsConstExpr<'ctx> for GetElementPtrConstExpr<'ctx> {
  fn as_const_expr(&self) -> ConstExpr<'ctx> {
    ConstExpr::GetElementPtr(self.clone())
  }
}

impl_const_expr_debug!(GetElementPtrConstExpr);

impl_as_constant_and_as_operand_for_const_expr!(GetElementPtrConstExpr);