use llvm_sys::core::LLVMGetInstructionOpcode;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMOpcode;

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
  Other(GenericValue<'ctx>),
}

impl<'ctx> HasDebugMetadata for Instruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for Instruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for Instruction<'ctx> {}

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
      _ => Instruction::Other(GenericValue::from_llvm(ptr)),
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
