use llvm_sys::core::{LLVMGetNumOperands, LLVMGetOperand, LLVMValueAsBasicBlock};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SwitchInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> InstructionDebugLoc for SwitchInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for SwitchInstruction<'ctx> {}

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
    let num_operands = unsafe { LLVMGetNumOperands(self.0) as usize };
    (num_operands - 2) / 2
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
