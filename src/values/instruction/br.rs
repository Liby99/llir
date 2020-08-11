use llvm_sys::core::{LLVMGetNumOperands, LLVMGetOperand, LLVMValueAsBasicBlock};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::utils::*;
use crate::values::*;
use crate::{FromLLVMBlock, FromLLVMValue, ValueRef};

/// Container class for branch instruction.
///
/// Branch could either be conditional or unconditional, and will have different behavior
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BranchInstruction<'ctx> {
  Conditional(ConditionalBranchInstruction<'ctx>),
  Unconditional(UnconditionalBranchInstruction<'ctx>),
}

impl<'ctx> GetDebugMetadata<'ctx> for BranchInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for BranchInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for BranchInstruction<'ctx> {}

impl<'ctx> AsInstruction<'ctx> for BranchInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Branch(*self)
  }
}

impl<'ctx> BranchInstruction<'ctx> {
  /// Get the target blocks from this branch instruction
  ///
  /// If this instruction is conditional, then there will be two target blocks;
  /// if unconditional, then there will be only one target block
  pub fn target_blocks(&self) -> Vec<Block<'ctx>> {
    match self {
      Self::Conditional(c) => vec![c.then_block(), c.else_block()],
      Self::Unconditional(u) => vec![u.target_block()],
    }
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

/// Conditional Branch Instruction
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ConditionalBranchInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> GetDebugMetadata<'ctx> for ConditionalBranchInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for ConditionalBranchInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for ConditionalBranchInstruction<'ctx> {}

impl<'ctx> ConditionalBranchInstruction<'ctx> {
  /// Get the condition operand used to determine which branch to take
  pub fn condition(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  /// Get the block taken when condition is satisfied (then block)
  pub fn then_block(&self) -> Block<'ctx> {
    let operand = unsafe { LLVMGetOperand(self.0, 2) };
    let block = unsafe { LLVMValueAsBasicBlock(operand) };
    Block::from_llvm(block)
  }

  /// Get the block taken when condition is unsatisfied (else block)
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

/// Unconditional branch instruction
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct UnconditionalBranchInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> GetDebugMetadata<'ctx> for UnconditionalBranchInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for UnconditionalBranchInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for UnconditionalBranchInstruction<'ctx> {}

impl<'ctx> UnconditionalBranchInstruction<'ctx> {
  /// Get the target block that this branch jumps to
  pub fn target_block(&self) -> Block<'ctx> {
    let operand = unsafe { LLVMGetOperand(self.0, 0) };
    let block = unsafe { LLVMValueAsBasicBlock(operand) };
    Block::from_llvm(block)
  }

  /// Check if this unconditional branch is jumping as a end-of-loop-body jump
  pub fn is_loop_jump(&self) -> Option<bool> {
    mdkind_ids::dbg_metadata(self.value_ref()).map(|_| mdkind_ids::loop_metadata(self.value_ref()).is_some())
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
