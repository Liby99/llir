use llvm_sys::core::{LLVMGetConstOpcode, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Unary constant expression
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct UnaryConstExpr<'ctx>(UnaryOpcode, LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for UnaryConstExpr<'ctx> {}

unsafe impl<'ctx> Sync for UnaryConstExpr<'ctx> {}

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

impl<'ctx> FromLLVMValue for UnaryConstExpr<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    let opcode = UnaryOpcode::from_llvm(unsafe { LLVMGetConstOpcode(ptr) }).unwrap();
    Self(opcode, ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for UnaryConstExpr<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.1
  }
}

impl<'ctx> AsConstExpr<'ctx> for UnaryConstExpr<'ctx> {
  fn as_const_expr(&self) -> ConstExpr<'ctx> {
    ConstExpr::Unary(self.clone())
  }
}
