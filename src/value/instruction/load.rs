use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::super::{Operand, FromLLVM, ValueRef};

#[derive(Copy, Clone)]
pub struct LoadInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> LoadInstruction<'ctx> {
  pub fn location(&self) -> Operand<'ctx> {
    // TODO
    Operand::Metadata
  }
}

impl<'ctx> FromLLVM for LoadInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    LoadInstruction(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for LoadInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
