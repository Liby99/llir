use std::marker::PhantomData;
use llvm_sys::prelude::LLVMValueRef;

use super::super::Operand;

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