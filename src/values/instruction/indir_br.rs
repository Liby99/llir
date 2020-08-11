use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// [Indirect Branch instruction](https://llvm.org/docs/LangRef.html#indirectbr-instruction)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IndirectBranchInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for IndirectBranchInstruction<'ctx> {}

unsafe impl<'ctx> Sync for IndirectBranchInstruction<'ctx> {}

impl<'ctx> GetType<'ctx> for IndirectBranchInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for IndirectBranchInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for IndirectBranchInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for IndirectBranchInstruction<'ctx> {}

impl<'ctx> IndirectBranchInstruction<'ctx> {
  /// Get the block address
  pub fn address(&self) -> BlockAddress<'ctx> {
    BlockAddress::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  /// Get the possible block destinations
  pub fn destinations(&self) -> Vec<Block<'ctx>> {
    let num_operands = unsafe { LLVMGetNumOperands(self.0) as u32 };
    let num_dests = num_operands - 1;
    (1..=num_dests)
      .map(|i| Block::from_llvm(unsafe { LLVMValueAsBasicBlock(LLVMGetOperand(self.0, i)) }))
      .collect()
  }
}

impl<'ctx> AsInstruction<'ctx> for IndirectBranchInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::IndirectBranch(*self)
  }
}

impl<'ctx> FromLLVMValue for IndirectBranchInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for IndirectBranchInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
