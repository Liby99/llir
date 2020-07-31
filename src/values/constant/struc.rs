use llvm_sys::core::{LLVMGetOperand, LLVMCountStructElementTypes, LLVMTypeOf};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::Constant;
use crate::{FromLLVMValue, ValueRef};

#[derive(Copy, Clone)]
pub struct StructConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> StructConstant<'ctx> {
  pub fn num_elements(&self) -> usize {
    let n = unsafe { LLVMCountStructElementTypes(LLVMTypeOf(self.0)) };
    n as usize
  }

  pub fn elements(&self) -> Vec<Constant<'ctx>> {
    (0..self.num_elements() as u32)
      .map(|i| Constant::from_llvm(unsafe { LLVMGetOperand(self.0, i) }))
      .collect()
  }
}

impl<'ctx> FromLLVMValue for StructConstant<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for StructConstant<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
