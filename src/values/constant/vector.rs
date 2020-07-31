use llvm_sys::core::{LLVMGetNumOperands, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::types::*;
use crate::*;

#[derive(Copy, Clone)]
pub struct VectorConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> HasType for VectorConstant<'ctx> {}

impl<'ctx> VectorConstant<'ctx> {
  pub fn num_elements(&self) -> usize {
    unsafe { LLVMGetNumOperands(self.0) as usize }
  }

  pub fn elements(&self) -> Vec<Constant<'ctx>> {
    (0..self.num_elements() as u32)
      .map(|i| Constant::from_llvm(unsafe { LLVMGetOperand(self.0, i) }))
      .collect()
  }

  pub fn get_vector_type(&self) -> VectorType<'ctx> {
    VectorType::from_llvm(self.get_type().type_ref())
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
