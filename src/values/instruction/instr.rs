use llvm_sys::core::{
  LLVMGetInstructionOpcode, LLVMGetInstructionParent, LLVMGetNextInstruction, LLVMGetNumOperands, LLVMGetOperand,
  LLVMGetPreviousInstruction,
};
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMOpcode;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Instruction<'ctx> {
  Alloca(AllocaInstruction<'ctx>),
  Binary(BinaryInstruction<'ctx>),
  Branch(BranchInstruction<'ctx>),
  Call(CallInstruction<'ctx>),
  FCmp(FCmpInstruction<'ctx>),
  GetElementPtr(GetElementPtrInstruction<'ctx>),
  ICmp(ICmpInstruction<'ctx>),
  Load(LoadInstruction<'ctx>),
  Phi(PhiInstruction<'ctx>),
  Return(ReturnInstruction<'ctx>),
  Store(StoreInstruction<'ctx>),
  Switch(SwitchInstruction<'ctx>),
  Unary(UnaryInstruction<'ctx>),
  Unreachable(UnreachableInstruction<'ctx>),
  Other(GenericInstruction<'ctx>),
}

impl<'ctx> InstructionDebugLoc for Instruction<'ctx> {}

impl<'ctx> Instruction<'ctx> {
  pub fn parent_block(&self) -> Block<'ctx> {
    let value = unsafe { LLVMGetInstructionParent(self.value_ref()) };
    Block::from_llvm(value)
  }

  pub fn prev_instruction(&self) -> Option<Self> {
    let this_ptr = self.value_ref();
    let prev_ptr = unsafe { LLVMGetPreviousInstruction(this_ptr) };
    if prev_ptr.is_null() {
      None
    } else {
      Some(Instruction::from_llvm(prev_ptr))
    }
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

  pub fn num_operands(&self) -> usize {
    unsafe { LLVMGetNumOperands(self.value_ref()) as usize }
  }

  pub fn operand(&self, index: usize) -> Option<Operand<'ctx>> {
    if index < self.num_operands() {
      Some(Operand::from_llvm(unsafe {
        LLVMGetOperand(self.value_ref(), index as u32)
      }))
    } else {
      None
    }
  }
}

impl<'ctx> FromLLVMValue for Instruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    match unsafe { LLVMGetInstructionOpcode(ptr) } {
      LLVMOpcode::LLVMAlloca => Instruction::Alloca(AllocaInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMBr => Instruction::Branch(BranchInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMCall => Instruction::Call(CallInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMFCmp => Instruction::FCmp(FCmpInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMGetElementPtr => Instruction::GetElementPtr(GetElementPtrInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMICmp => Instruction::ICmp(ICmpInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMLoad => Instruction::Load(LoadInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMPHI => Instruction::Phi(PhiInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMRet => Instruction::Return(ReturnInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMStore => Instruction::Store(StoreInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMSwitch => Instruction::Switch(SwitchInstruction::from_llvm(ptr)),
      LLVMOpcode::LLVMUnreachable => Instruction::Unreachable(UnreachableInstruction::from_llvm(ptr)),
      op if BinaryOpcode::from_llvm(op).is_some() => Instruction::Binary(BinaryInstruction::from_llvm(ptr)),
      op if UnaryOpcode::from_llvm(op).is_some() => Instruction::Unary(UnaryInstruction::from_llvm(ptr)),
      _ => Instruction::Other(GenericInstruction::from_llvm(ptr)),
    }
  }
}

impl<'ctx> ValueRef for Instruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    match self {
      Instruction::Alloca(alc_instr) => alc_instr.value_ref(),
      Instruction::Binary(bin_instr) => bin_instr.value_ref(),
      Instruction::Branch(br_instr) => br_instr.value_ref(),
      Instruction::Call(call_instr) => call_instr.value_ref(),
      Instruction::FCmp(fcmp_instr) => fcmp_instr.value_ref(),
      Instruction::GetElementPtr(gep_instr) => gep_instr.value_ref(),
      Instruction::ICmp(icmp_instr) => icmp_instr.value_ref(),
      Instruction::Load(ld_instr) => ld_instr.value_ref(),
      Instruction::Phi(phi_instr) => phi_instr.value_ref(),
      Instruction::Return(ret_instr) => ret_instr.value_ref(),
      Instruction::Store(st_instr) => st_instr.value_ref(),
      Instruction::Switch(switch_instr) => switch_instr.value_ref(),
      Instruction::Unary(una_instr) => una_instr.value_ref(),
      Instruction::Unreachable(unr_instr) => unr_instr.value_ref(),
      Instruction::Other(otr_instr) => otr_instr.value_ref(),
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GenericInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> InstructionDebugLoc for GenericInstruction<'ctx> {}

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
