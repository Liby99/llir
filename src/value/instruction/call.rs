use std::marker::PhantomData;
use llvm_sys::prelude::LLVMValueRef;

use super::super::{Function, Operand};

#[derive(Copy, Clone)]
pub struct CallInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> CallInstruction<'ctx> {
  pub fn callee_function(&self) -> Option<Function<'ctx>> {
    // TODO
    None
  }

  pub fn callee(&self) -> Operand<'ctx> {
    // TODO
    Operand::Metadata
  }

  pub fn num_arguments(&self) -> usize {
    // TODO
    0
  }

  pub fn args(&self) -> Vec<Operand<'ctx>> {
    // TODO
    vec![]
  }
}