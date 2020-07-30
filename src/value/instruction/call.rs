use llvm_sys::core::{LLVMIsTailCall, LLVMGetNumOperands};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::super::{Function, Operand, ValueRef};

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
    let num_operands = unsafe { LLVMGetNumOperands(self.0) };
    num_operands as usize - 1
  }

  pub fn args(&self) -> Vec<Operand<'ctx>> {
    // TODO
    vec![]
  }

  pub fn is_tail_call(&self) -> bool {
    unsafe { LLVMIsTailCall(self.0) == 1 }
  }
}

impl<'ctx> ValueRef for CallInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
