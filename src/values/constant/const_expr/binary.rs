use llvm_sys::core::{LLVMGetConstOpcode, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Binary constant expression
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BinaryConstExpr<'ctx>(BinaryOpcode, LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for BinaryConstExpr<'ctx> {}

unsafe impl<'ctx> Sync for BinaryConstExpr<'ctx> {}

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

impl<'ctx> FromLLVMValue for BinaryConstExpr<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    let opcode = BinaryOpcode::from_llvm(unsafe { LLVMGetConstOpcode(ptr) }).unwrap();
    Self(opcode, ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for BinaryConstExpr<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.1
  }
}

impl<'ctx> AsConstExpr<'ctx> for BinaryConstExpr<'ctx> {
  fn as_const_expr(&self) -> ConstExpr<'ctx> {
    ConstExpr::Binary(self.clone())
  }
}
