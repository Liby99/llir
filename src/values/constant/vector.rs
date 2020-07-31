use llvm_sys::core::{LLVMGetNumOperands, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::Constant;
use crate::{FromLLVMValue, ValueRef};

#[derive(Copy, Clone)]
pub struct VectorConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> VectorConstant<'ctx> {
  pub fn num_elements(&self) -> usize {
    let n = unsafe { LLVMGetNumOperands(self.0) };
    n as usize
  }

  pub fn elements(&self) -> Vec<Constant<'ctx>> {
    (0..self.num_elements() as u32)
      .map(|i| Constant::from_llvm(unsafe { LLVMGetOperand(self.0, i) }))
      .collect()
  }
}

impl<'ctx> FromLLVMValue for VectorConstant<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for VectorConstant<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
