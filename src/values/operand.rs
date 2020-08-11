use llvm_sys::core::LLVMGetValueKind;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMValueKind;

use crate::values::*;
use crate::*;

/// Container class for operand
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Operand<'ctx> {
  /// Result from a previous local instruction as operand
  Instruction(Instruction<'ctx>),
  /// Function argument as operand
  Argument(Argument<'ctx>),
  /// Constant as operand
  Constant(Constant<'ctx>),
  /// Inline Assembly as operand
  InlineAsm(InlineAsm<'ctx>),
  /// Metadata as operand
  Metadata(Metadata<'ctx>),
}

impl<'ctx> GetType<'ctx> for Operand<'ctx> {}

impl<'ctx> FromLLVMValue for Operand<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    use LLVMValueKind::*;
    match unsafe { LLVMGetValueKind(ptr) } {
      LLVMArgumentValueKind => Self::Argument(Argument::from_llvm(ptr)),
      LLVMInstructionValueKind => Self::Instruction(Instruction::from_llvm(ptr)),
      LLVMMetadataAsValueValueKind => Self::Metadata(Metadata::from_llvm(ptr)),
      LLVMInlineAsmValueKind => Self::InlineAsm(InlineAsm::from_llvm(ptr)),
      _ => Self::Constant(Constant::from_llvm(ptr)),
    }
  }
}

impl<'ctx> ValueRef for Operand<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    match self {
      Operand::Instruction(instr) => instr.value_ref(),
      Operand::Argument(arg) => arg.value_ref(),
      Operand::Constant(constant) => constant.value_ref(),
      Operand::InlineAsm(asm) => asm.value_ref(),
      Operand::Metadata(metadata) => metadata.value_ref(),
    }
  }
}

impl<'ctx> AsOperand<'ctx> for Operand<'ctx> {
  fn as_operand(&self) -> Operand<'ctx> {
    self.clone()
  }
}
