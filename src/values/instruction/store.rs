use llvm_sys::core::LLVMGetOperand;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::super::Operand;
use crate::{FromLLVMValue, ValueRef};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StoreInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> StoreInstruction<'ctx> {
  pub fn location(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 1) })
  }

  pub fn value(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }
}

impl<'ctx> FromLLVMValue for StoreInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    StoreInstruction(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for StoreInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
