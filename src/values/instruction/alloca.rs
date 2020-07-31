use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

#[derive(Debug, Copy, Clone)]
pub struct AllocaInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> HasType for AllocaInstruction<'ctx> {}

impl<'ctx> AllocaInstruction<'ctx> {
  pub fn get_pointer_type(&self) -> PointerType<'ctx> {
    PointerType::from_llvm(self.get_type().type_ref())
  }
}

impl<'ctx> FromLLVMValue for AllocaInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    AllocaInstruction(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for AllocaInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}