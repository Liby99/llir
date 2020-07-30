use std::marker::PhantomData;
use llvm_sys::prelude::LLVMValueRef;

use super::super::Operand;

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