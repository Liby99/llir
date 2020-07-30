use std::marker::PhantomData;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::core::{LLVMGetOperand, LLVMValueAsBasicBlock};

use super::super::{Operand, Block, Constant};

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