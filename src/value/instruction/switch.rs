use llvm_sys::core::{LLVMGetNumOperands, LLVMGetOperand, LLVMValueAsBasicBlock};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::super::{Block, Constant, Operand};
use crate::{FromLLVMBlock, FromLLVMValue, ValueRef};

#[derive(Debug, Copy, Clone)]
pub struct SwitchInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> SwitchInstruction<'ctx> {
  pub fn condition(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  pub fn default_block(&self) -> Block<'ctx> {
    let operand = unsafe { LLVMGetOperand(self.0, 1) };
    let block = unsafe { LLVMValueAsBasicBlock(operand) };
    Block::from_llvm(block)
  }

  pub fn num_branches(&self) -> usize {
    let num_operands = unsafe { LLVMGetNumOperands(self.0) };
    ((num_operands - 2) / 2) as usize
  }

  pub fn branches(&self) -> Vec<(Constant<'ctx>, Block<'ctx>)> {
    (0..self.num_branches() as u32)
      .map(|i| {
        (
          Constant::from_llvm(unsafe { LLVMGetOperand(self.0, i * 2 + 2) }),
          Block::from_llvm(unsafe { LLVMValueAsBasicBlock(LLVMGetOperand(self.0, i * 2 + 3)) }),
        )
      })
      .collect()
  }
}

impl<'ctx> FromLLVMValue for SwitchInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for SwitchInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
