use llvm_sys::core::{LLVMGetOperand, LLVMValueAsBasicBlock};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::super::{Block, Operand};
use crate::{FromLLVMBlock, FromLLVMValue, ValueRef};

#[derive(Copy, Clone)]
pub struct ConditionalBranchInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> ConditionalBranchInstruction<'ctx> {
  pub fn condition(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  pub fn then_block(&self) -> Block<'ctx> {
    let operand = unsafe { LLVMGetOperand(self.0, 2) };
    let block = unsafe { LLVMValueAsBasicBlock(operand) };
    Block::from_llvm(block)
  }

  pub fn else_block(&self) -> Block<'ctx> {
    let operand = unsafe { LLVMGetOperand(self.0, 1) };
    let block = unsafe { LLVMValueAsBasicBlock(operand) };
    Block::from_llvm(block)
  }
}

impl<'ctx> FromLLVMValue for ConditionalBranchInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    ConditionalBranchInstruction(ptr, PhantomData)
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
    Block::from_llvm(block)
  }
}

impl<'ctx> ValueRef for UnconditionalBranchInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> FromLLVMValue for UnconditionalBranchInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    UnconditionalBranchInstruction(ptr, PhantomData)
  }
}
