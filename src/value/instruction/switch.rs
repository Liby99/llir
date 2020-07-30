use llvm_sys::core::{LLVMGetOperand, LLVMValueAsBasicBlock};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::super::{Block, Constant, Operand, ValueRef};

#[derive(Copy, Clone)]
pub struct SwitchInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> SwitchInstruction<'ctx> {
  pub fn condition(&self) -> Operand<'ctx> {
    // TODO
    Operand::Metadata
  }

  pub fn default_block(&self) -> Block<'ctx> {
    let operand = unsafe { LLVMGetOperand(self.0, 1) };
    let block = unsafe { LLVMValueAsBasicBlock(operand) };
    Block(block, PhantomData)
  }

  pub fn num_branches(&self) -> usize {
    // TODO
    0
  }

  pub fn branches(&self) -> Vec<(Constant<'ctx>, Block<'ctx>)> {
    // TODO
    vec![]
  }
}

impl<'ctx> ValueRef for SwitchInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
