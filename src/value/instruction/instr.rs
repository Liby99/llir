use llvm_sys::core::LLVMGetInstructionParent;
use llvm_sys::prelude::LLVMValueRef;

use super::super::{Block, ValueRef};
use super::*;

#[derive(Copy, Clone)]
pub enum Instruction<'ctx> {
  Binary(BinaryInstruction<'ctx>),
  Unary(UnaryInstruction<'ctx>),
  Call(CallInstruction<'ctx>),
  ConditionalBranch(ConditionalBranchInstruction<'ctx>),
  UnconditionalBranch(UnconditionalBranchInstruction<'ctx>),
  Switch(SwitchInstruction<'ctx>),
  Return(ReturnInstruction<'ctx>),
  Alloca(AllocaInstruction<'ctx>),
  Load(LoadInstruction<'ctx>),
  Store(StoreInstruction<'ctx>),
}

impl<'ctx> Instruction<'ctx> {
  pub fn from_llvm(_ptr: LLVMValueRef) -> Option<Self> {
    // TODO
    None
  }

  pub fn parent_block(&self) -> Block<'ctx> {
    let value = unsafe { LLVMGetInstructionParent(self.value_ref()) };
    Block::from_llvm(value).unwrap()
  }
}

impl<'ctx> ValueRef for Instruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    match self {
      Instruction::Binary(bin_instr) => bin_instr.value_ref(),
      Instruction::Unary(una_instr) => una_instr.value_ref(),
      Instruction::Call(call_instr) => call_instr.value_ref(),
      Instruction::ConditionalBranch(cbr_instr) => cbr_instr.value_ref(),
      Instruction::UnconditionalBranch(ubr_instr) => ubr_instr.value_ref(),
      Instruction::Switch(switch_instr) => switch_instr.value_ref(),
      Instruction::Return(ret_instr) => ret_instr.value_ref(),
      Instruction::Alloca(alc_instr) => alc_instr.value_ref(),
      Instruction::Load(ld_instr) => ld_instr.value_ref(),
      Instruction::Store(st_instr) => st_instr.value_ref(),
    }
  }
}
