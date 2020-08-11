use llvm_sys::core::{LLVMGetNumOperands, LLVMGetOperand, LLVMValueAsBasicBlock};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// One Case of switch
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SwitchCase<'ctx> {
  /// The value that the switch operand will be checked against.
  /// This value is always integer constant.
  pub case: IntConstant<'ctx>,
  /// The block it will take when the value matches
  pub block: Block<'ctx>,
}

/// Switch instruction
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SwitchInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for SwitchInstruction<'ctx> {}

unsafe impl<'ctx> Sync for SwitchInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for SwitchInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for SwitchInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for SwitchInstruction<'ctx> {}

impl<'ctx> SwitchInstruction<'ctx> {
  /// Get the value that is checked against
  pub fn condition(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  /// Get the default block, which is the block this instruction will jump to when
  /// no branch target is matching the value
  pub fn default_block(&self) -> Block<'ctx> {
    let operand = unsafe { LLVMGetOperand(self.0, 1) };
    let block = unsafe { LLVMValueAsBasicBlock(operand) };
    Block::from_llvm(block)
  }

  /// Get the number of cases
  pub fn num_cases(&self) -> usize {
    let num_operands = unsafe { LLVMGetNumOperands(self.0) as usize };
    (num_operands - 2) / 2
  }

  /// Get the cases
  pub fn cases(&self) -> Vec<SwitchCase<'ctx>> {
    (0..self.num_cases() as u32)
      .map(|i| SwitchCase {
        case: IntConstant::from_llvm(unsafe { LLVMGetOperand(self.0, i * 2 + 2) }),
        block: Block::from_llvm(unsafe { LLVMValueAsBasicBlock(LLVMGetOperand(self.0, i * 2 + 3)) }),
      })
      .collect()
  }
}

impl<'ctx> AsInstruction<'ctx> for SwitchInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Switch(*self)
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
