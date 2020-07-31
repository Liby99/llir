use llvm_sys::core::{LLVMGetArrayLength, LLVMGetOperand, LLVMTypeOf};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::Constant;
use crate::*;
use crate::values::*;
use crate::types::*;

#[derive(Copy, Clone)]
pub struct ArrayConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> HasType for ArrayConstant<'ctx> {}

impl<'ctx> ArrayConstant<'ctx> {
  pub fn num_elements(&self) -> usize {
    unsafe { LLVMGetArrayLength(LLVMTypeOf(self.0)) as usize }
  }

  pub fn elements(&self) -> Vec<Constant<'ctx>> {
    (0..self.num_elements() as u32)
      .map(|i| Constant::from_llvm(unsafe { LLVMGetOperand(self.0, i) }))
      .collect()
  }

  pub fn get_array_type(&self) -> ArrayType<'ctx> {
    ArrayType::from_llvm(self.get_type().type_ref())
  }
}

impl<'ctx> FromLLVMValue for ArrayConstant<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for ArrayConstant<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
