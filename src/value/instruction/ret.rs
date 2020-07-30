use std::marker::PhantomData;
use llvm_sys::prelude::LLVMValueRef;

use super::super::Operand;

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