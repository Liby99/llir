use llvm_sys::core::{LLVMGetNumOperands, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

#[derive(Debug, Copy, Clone)]
pub struct GetElementPtrInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> HasType for GetElementPtrInstruction<'ctx> {}

impl<'ctx> GetElementPtrInstruction<'ctx> {
  pub fn location(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  pub fn num_indices(&self) -> usize {
    (unsafe { LLVMGetNumOperands(self.0) as usize }) - 1
  }

  pub fn indices(&self) -> Vec<Operand<'ctx>> {
    (0..self.num_indices())
      .map(|i| Operand::from_llvm(unsafe { LLVMGetOperand(self.0, i as u32) }))
      .collect()
  }
}

impl<'ctx> FromLLVMValue for GetElementPtrInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    GetElementPtrInstruction(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for GetElementPtrInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
