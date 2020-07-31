use llvm_sys::core::{LLVMGetInstructionOpcode, LLVMGetInstructionParent, LLVMGetNumOperands};
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMOpcode;
use std::marker::PhantomData;

use super::super::{Block, FromLLVM, ValueRef};
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
  Other(GenericInstruction<'ctx>),
}

impl<'ctx> Instruction<'ctx> {
  pub fn parent_block(&self) -> Block<'ctx> {
    let value = unsafe { LLVMGetInstructionParent(self.value_ref()) };
    Block::from_llvm(value)
  }
}

impl<'ctx> FromLLVM for Instruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    match unsafe { LLVMGetInstructionOpcode(ptr) } {
      LLVMOpcode::LLVMCall => Instruction::Call(CallInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMBr => match unsafe { LLVMGetNumOperands(ptr) } {
        1 => Instruction::UnconditionalBranch(UnconditionalBranchInstruction::from_llvm(ptr)),
        3 => Instruction::ConditionalBranch(ConditionalBranchInstruction::from_llvm(ptr)),
        _ => panic!("Unknown branch variant"),
      },
      LLVMOpcode::LLVMSwitch => Instruction::Switch(SwitchInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMRet => Instruction::Return(ReturnInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMAlloca => Instruction::Alloca(AllocaInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMLoad => Instruction::Load(LoadInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMStore => Instruction::Store(StoreInstruction::from_llvm(ptr)),
      opcode if BinaryOpcode::from_llvm(opcode).is_some() => Instruction::Binary(BinaryInstruction::from_llvm(ptr)),
      opcode if UnaryOpcode::from_llvm(opcode).is_some() => Instruction::Unary(UnaryInstruction::from_llvm(ptr)),
      _ => Instruction::Other(GenericInstruction::from_llvm(ptr)),
    }
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
      Instruction::Other(otr_instr) => otr_instr.value_ref(),
    }
  }
}

#[derive(Copy, Clone)]
pub struct GenericInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> ValueRef for GenericInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> FromLLVM for GenericInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}
