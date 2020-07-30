use std::marker::PhantomData;
use llvm_sys::prelude::LLVMValueRef;

use super::super::Operand;

#[derive(Copy, Clone)]
pub struct LoadInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> LoadInstruction<'ctx> {
  pub fn location(&self) -> Operand<'ctx> {
    // TODO
    Operand::Metadata
  }
}