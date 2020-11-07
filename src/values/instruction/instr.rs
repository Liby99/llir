use llvm_sys::core::LLVMGetInstructionOpcode;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMOpcode;

use crate::values::*;
use crate::*;

/// [Instruction](https://llvm.org/docs/LangRef.html#instruction-reference)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Instruction<'ctx> {
  Alloca(AllocaInstruction<'ctx>),
  Binary(BinaryInstruction<'ctx>),
  Branch(BranchInstruction<'ctx>),
  Call(CallInstruction<'ctx>),
  CallBr(CallBrInstruction<'ctx>),
  ExtractValue(ExtractValueInstruction<'ctx>),
  FCmp(FCmpInstruction<'ctx>),
  GetElementPtr(GetElementPtrInstruction<'ctx>),
  ICmp(ICmpInstruction<'ctx>),
  IndirectBranch(IndirectBranchInstruction<'ctx>),
  InsertValue(InsertValueInstruction<'ctx>),
  Load(LoadInstruction<'ctx>),
  Phi(PhiInstruction<'ctx>),
  Return(ReturnInstruction<'ctx>),
  Select(SelectInstruction<'ctx>),
  Store(StoreInstruction<'ctx>),
  Switch(SwitchInstruction<'ctx>),
  Unary(UnaryInstruction<'ctx>),
  Unreachable(UnreachableInstruction<'ctx>),
  Other(GenericValue<'ctx>),
}

impl<'ctx> GetDebugMetadata<'ctx> for Instruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for Instruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for Instruction<'ctx> {}

impl<'ctx> ValueOpcode for Instruction<'ctx> {
  fn opcode(&self) -> Opcode {
    match self {
      Self::Alloca(alc_instr) => alc_instr.opcode(),
      Self::Binary(bin_instr) => bin_instr.opcode(),
      Self::Branch(br_instr) => br_instr.opcode(),
      Self::Call(call_instr) => call_instr.opcode(),
      Self::CallBr(call_br_instr) => call_br_instr.opcode(),
      Self::ExtractValue(extval_instr) => extval_instr.opcode(),
      Self::FCmp(fcmp_instr) => fcmp_instr.opcode(),
      Self::GetElementPtr(gep_instr) => gep_instr.opcode(),
      Self::ICmp(icmp_instr) => icmp_instr.opcode(),
      Self::IndirectBranch(indbr_instr) => indbr_instr.opcode(),
      Self::InsertValue(insval_instr) => insval_instr.opcode(),
      Self::Load(ld_instr) => ld_instr.opcode(),
      Self::Phi(phi_instr) => phi_instr.opcode(),
      Self::Return(ret_instr) => ret_instr.opcode(),
      Self::Select(sel_instr) => sel_instr.opcode(),
      Self::Store(st_instr) => st_instr.opcode(),
      Self::Switch(switch_instr) => switch_instr.opcode(),
      Self::Unary(una_instr) => una_instr.opcode(),
      Self::Unreachable(unr_instr) => unr_instr.opcode(),
      Self::Other(_) => Opcode::Unknown,
    }
  }
}

impl<'ctx> AsInstruction<'ctx> for Instruction<'ctx> {
  fn as_instruction(&self) -> Self {
    self.clone()
  }
}

impl_as_operand_for_instr!(Instruction);

impl<'ctx> FromLLVMValue for Instruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    use LLVMOpcode::*;
    match unsafe { LLVMGetInstructionOpcode(ptr) } {
      LLVMAlloca => Self::Alloca(AllocaInstruction::from_llvm(ptr)),
      LLVMBr => Self::Branch(BranchInstruction::from_llvm(ptr)),
      LLVMCall => Self::Call(CallInstruction::from_llvm(ptr)),
      LLVMCallBr => Self::CallBr(CallBrInstruction::from_llvm(ptr)),
      LLVMExtractValue => Self::ExtractValue(ExtractValueInstruction::from_llvm(ptr)),
      LLVMFCmp => Self::FCmp(FCmpInstruction::from_llvm(ptr)),
      LLVMGetElementPtr => Self::GetElementPtr(GetElementPtrInstruction::from_llvm(ptr)),
      LLVMICmp => Self::ICmp(ICmpInstruction::from_llvm(ptr)),
      LLVMIndirectBr => Self::IndirectBranch(IndirectBranchInstruction::from_llvm(ptr)),
      LLVMLoad => Self::Load(LoadInstruction::from_llvm(ptr)),
      LLVMPHI => Self::Phi(PhiInstruction::from_llvm(ptr)),
      LLVMRet => Self::Return(ReturnInstruction::from_llvm(ptr)),
      LLVMSelect => Self::Select(SelectInstruction::from_llvm(ptr)),
      LLVMStore => Self::Store(StoreInstruction::from_llvm(ptr)),
      LLVMSwitch => Self::Switch(SwitchInstruction::from_llvm(ptr)),
      LLVMUnreachable => Self::Unreachable(UnreachableInstruction::from_llvm(ptr)),
      op if BinaryOpcode::from_llvm(op).is_some() => Self::Binary(BinaryInstruction::from_llvm(ptr)),
      op if UnaryOpcode::from_llvm(op).is_some() => Self::Unary(UnaryInstruction::from_llvm(ptr)),
      _ => Self::Other(GenericValue::from_llvm(ptr)),
    }
  }
}

impl<'ctx> ValueRef for Instruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    match self {
      Self::Alloca(alc_instr) => alc_instr.value_ref(),
      Self::Binary(bin_instr) => bin_instr.value_ref(),
      Self::Branch(br_instr) => br_instr.value_ref(),
      Self::Call(call_instr) => call_instr.value_ref(),
      Self::CallBr(call_br_instr) => call_br_instr.value_ref(),
      Self::ExtractValue(extval_instr) => extval_instr.value_ref(),
      Self::FCmp(fcmp_instr) => fcmp_instr.value_ref(),
      Self::GetElementPtr(gep_instr) => gep_instr.value_ref(),
      Self::ICmp(icmp_instr) => icmp_instr.value_ref(),
      Self::IndirectBranch(indbr_instr) => indbr_instr.value_ref(),
      Self::InsertValue(insval_instr) => insval_instr.value_ref(),
      Self::Load(ld_instr) => ld_instr.value_ref(),
      Self::Phi(phi_instr) => phi_instr.value_ref(),
      Self::Return(ret_instr) => ret_instr.value_ref(),
      Self::Select(sel_instr) => sel_instr.value_ref(),
      Self::Store(st_instr) => st_instr.value_ref(),
      Self::Switch(switch_instr) => switch_instr.value_ref(),
      Self::Unary(una_instr) => una_instr.value_ref(),
      Self::Unreachable(unr_instr) => unr_instr.value_ref(),
      Self::Other(otr_instr) => otr_instr.value_ref(),
    }
  }
}
