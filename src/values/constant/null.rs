use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

#[derive(Copy, Clone)]
pub struct NullConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> HasType for NullConstant<'ctx> {}

impl<'ctx> NullConstant<'ctx> {
  pub fn get_pointer_type(&self) -> PointerType<'ctx> {
    PointerType::from_llvm(self.get_type().type_ref())
  }
}

impl<'ctx> ValueRef for NullConstant<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> FromLLVMValue for NullConstant<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}
