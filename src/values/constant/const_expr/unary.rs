use llvm_sys::core::{LLVMGetConstOpcode, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

#[derive(Copy, Clone)]
pub struct UnaryConstExpr<'ctx>(UnaryOpcode, LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> UnaryConstExpr<'ctx> {
  pub fn opcode(&self) -> UnaryOpcode {
    self.0
  }

  pub fn op0(&self) -> Constant<'ctx> {
    Constant::from_llvm(unsafe { LLVMGetOperand(self.1, 0) })
  }
}

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
