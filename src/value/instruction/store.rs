use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::super::Operand;
use crate::{FromLLVMValue, ValueRef};

#[derive(Copy, Clone)]
pub struct StoreInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> StoreInstruction<'ctx> {
  pub fn location(&self) -> Operand<'ctx> {
    // TODO
    Operand::Metadata
  }

  pub fn value(&self) -> Operand<'ctx> {
    // TODO
    Operand::Metadata
  }
}

impl<'ctx> FromLLVMValue for StoreInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    StoreInstruction(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for StoreInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
