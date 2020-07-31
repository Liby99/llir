use llvm_sys::core::{LLVMCountStructElementTypes, LLVMGetOperand, LLVMTypeOf};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

#[derive(Copy, Clone)]
pub struct StructConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> HasType for StructConstant<'ctx> {}

impl<'ctx> StructConstant<'ctx> {
  pub fn num_elements(&self) -> usize {
    unsafe { LLVMCountStructElementTypes(LLVMTypeOf(self.0)) as usize }
  }

  pub fn elements(&self) -> Vec<Constant<'ctx>> {
    (0..self.num_elements() as u32)
      .map(|i| Constant::from_llvm(unsafe { LLVMGetOperand(self.0, i) }))
      .collect()
  }

  pub fn get_struct_type(&self) -> StructType<'ctx> {
    StructType::from_llvm(self.get_type().type_ref())
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
