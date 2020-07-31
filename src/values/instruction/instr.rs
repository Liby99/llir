use llvm_sys::core::{LLVMGetInstructionOpcode, LLVMGetInstructionParent, LLVMGetNextInstruction};
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMOpcode;
use std::marker::PhantomData;

use super::super::Block;
use super::*;
use crate::{FromLLVMBlock, FromLLVMValue, ValueRef};

#[derive(Debug, Copy, Clone)]
pub enum Instruction<'ctx> {
  Binary(BinaryInstruction<'ctx>),
  Unary(UnaryInstruction<'ctx>),
  Call(CallInstruction<'ctx>),
  Branch(BranchInstruction<'ctx>),
  Switch(SwitchInstruction<'ctx>),
  Return(ReturnInstruction<'ctx>),
  Alloca(AllocaInstruction<'ctx>),
  Load(LoadInstruction<'ctx>),
  Store(StoreInstruction<'ctx>),
  GetElementPtr(GetElementPtrInstruction<'ctx>),
  Phi(PhiInstruction<'ctx>),
  Other(GenericInstruction<'ctx>),
}

impl<'ctx> Instruction<'ctx> {
  pub fn parent_block(&self) -> Block<'ctx> {
    let value = unsafe { LLVMGetInstructionParent(self.value_ref()) };
    Block::from_llvm(value)
  }

  pub fn next_instruction(&self) -> Option<Self> {
    let this_ptr = self.value_ref();
    let next_ptr = unsafe { LLVMGetNextInstruction(this_ptr) };
    if next_ptr.is_null() {
      None
    } else {
      Some(Instruction::from_llvm(next_ptr))
    }
  }
}

impl<'ctx> FromLLVMValue for Instruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    match unsafe { LLVMGetInstructionOpcode(ptr) } {
      LLVMOpcode::LLVMCall => Instruction::Call(CallInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMBr => Instruction::Branch(BranchInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMSwitch => Instruction::Switch(SwitchInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMRet => Instruction::Return(ReturnInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMAlloca => Instruction::Alloca(AllocaInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMLoad => Instruction::Load(LoadInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMStore => Instruction::Store(StoreInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMGetElementPtr => Instruction::GetElementPtr(GetElementPtrInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMPHI => Instruction::Phi(PhiInstruction::from_llvm(ptr)),
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
      Instruction::Branch(br_instr) => br_instr.value_ref(),
      Instruction::Switch(switch_instr) => switch_instr.value_ref(),
      Instruction::Return(ret_instr) => ret_instr.value_ref(),
      Instruction::Alloca(alc_instr) => alc_instr.value_ref(),
      Instruction::Load(ld_instr) => ld_instr.value_ref(),
      Instruction::Store(st_instr) => st_instr.value_ref(),
      Instruction::GetElementPtr(gep_instr) => gep_instr.value_ref(),
      Instruction::Phi(phi_instr) => phi_instr.value_ref(),
      Instruction::Other(otr_instr) => otr_instr.value_ref(),
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct GenericInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> ValueRef for GenericInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> FromLLVMValue for GenericInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}
