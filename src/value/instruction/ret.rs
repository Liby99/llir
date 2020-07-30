use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::super::{Operand, ValueRef};

#[derive(Copy, Clone)]
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

impl<'ctx> ValueRef for ReturnInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
