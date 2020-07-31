use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::super::Operand;
use crate::{FromLLVMValue, ValueRef};

#[derive(Debug, Copy, Clone)]
pub struct ReturnInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> ReturnInstruction<'ctx> {
  pub fn has_op(&self) -> bool {
    // TODO
    false
  }

  pub fn op(&self) -> Option<Operand<'ctx>> {
    // TODO
    None
  }
}

impl<'ctx> FromLLVMValue for ReturnInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for ReturnInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
