use llvm_sys::core::{LLVMGetOperand, LLVMValueAsBasicBlock};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::super::{Block, Operand, ValueRef};

#[derive(Copy, Clone)]
pub struct ConditionalBranchInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> ConditionalBranchInstruction<'ctx> {
  pub fn condition(&self) -> Operand<'ctx> {
    // TODO
    Operand::Metadata
  }

  pub fn then_block(&self) -> Block<'ctx> {
    let operand = unsafe { LLVMGetOperand(self.0, 2) };
    let block = unsafe { LLVMValueAsBasicBlock(operand) };
    Block(block, PhantomData)
  }

  pub fn else_block(&self) -> Block<'ctx> {
    let operand = unsafe { LLVMGetOperand(self.0, 1) };
    let block = unsafe { LLVMValueAsBasicBlock(operand) };
    Block(block, PhantomData)
  }
}

impl<'ctx> ValueRef for ConditionalBranchInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

#[derive(Copy, Clone)]
pub struct UnconditionalBranchInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> UnconditionalBranchInstruction<'ctx> {
  pub fn to_block(&self) -> Block<'ctx> {
    let operand = unsafe { LLVMGetOperand(self.0, 0) };
    let block = unsafe { LLVMValueAsBasicBlock(operand) };
    Block(block, PhantomData)
  }
}

impl<'ctx> ValueRef for UnconditionalBranchInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
