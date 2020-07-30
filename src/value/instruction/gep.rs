use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::super::{Operand, ValueRef};

#[derive(Copy, Clone)]
pub struct GetElementPtrInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> GetElementPtrInstruction<'ctx> {
  pub fn location(&self) -> Operand<'ctx> {
    // TODO
    Operand::Metadata
  }

  pub fn indices(&self) -> Vec<Operand<'ctx>> {
    // TODO
    vec![]
  }
}

impl<'ctx> ValueRef for GetElementPtrInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
