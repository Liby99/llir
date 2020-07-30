use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::super::ValueRef;

#[derive(Copy, Clone)]
pub struct AllocaInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> AllocaInstruction<'ctx> {}

impl<'ctx> ValueRef for AllocaInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
