use llvm_sys::core::{LLVMConstIntGetSExtValue, LLVMConstIntGetZExtValue};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

#[derive(Copy, Clone)]
pub struct IntConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> HasType for IntConstant<'ctx> {}

impl<'ctx> IntConstant<'ctx> {
  pub fn zext_value(&self) -> u64 {
    let val = unsafe { LLVMConstIntGetZExtValue(self.0) };
    val as u64
  }

  pub fn sext_value(&self) -> i64 {
    let val = unsafe { LLVMConstIntGetSExtValue(self.0) };
    val as i64
  }

  pub fn get_int_type(&self) -> IntType<'ctx> {
    IntType::from_llvm(self.get_type().type_ref())
  }
}

impl<'ctx> ValueRef for IntConstant<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> FromLLVMValue for IntConstant<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}
