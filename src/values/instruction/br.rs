use llvm_sys::core::{LLVMGetNumOperands, LLVMGetOperand, LLVMValueAsBasicBlock};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::{FromLLVMBlock, FromLLVMValue, ValueRef};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BranchInstruction<'ctx> {
  Conditional(ConditionalBranchInstruction<'ctx>),
  Unconditional(UnconditionalBranchInstruction<'ctx>),
}

impl<'ctx> HasDebugMetadata for BranchInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for BranchInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for BranchInstruction<'ctx> {}

impl<'ctx> AsInstruction<'ctx> for BranchInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Branch(*self)
  }
}

impl<'ctx> FromLLVMValue for BranchInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    match unsafe { LLVMGetNumOperands(ptr) } {
      1 => Self::Unconditional(UnconditionalBranchInstruction::from_llvm(ptr)),
      3 => Self::Conditional(ConditionalBranchInstruction::from_llvm(ptr)),
      _ => panic!("Unknown branch variant"),
    }
  }
}

impl<'ctx> ValueRef for BranchInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    match self {
      Self::Conditional(c) => c.value_ref(),
      Self::Unconditional(u) => u.value_ref(),
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ConditionalBranchInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> HasDebugMetadata for ConditionalBranchInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for ConditionalBranchInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for ConditionalBranchInstruction<'ctx> {}

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

impl<'ctx> AsInstruction<'ctx> for ConditionalBranchInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Branch(BranchInstruction::Conditional(*self))
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct UnconditionalBranchInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> HasDebugMetadata for UnconditionalBranchInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for UnconditionalBranchInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for UnconditionalBranchInstruction<'ctx> {}

impl<'ctx> UnconditionalBranchInstruction<'ctx> {
  pub fn target_block(&self) -> Block<'ctx> {
    let operand = unsafe { LLVMGetOperand(self.0, 0) };
    let block = unsafe { LLVMValueAsBasicBlock(operand) };
    Block::from_llvm(block)
  }
}

impl<'ctx> AsInstruction<'ctx> for UnconditionalBranchInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Branch(BranchInstruction::Unconditional(*self))
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
