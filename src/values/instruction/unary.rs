use llvm_sys::core::{LLVMGetInstructionOpcode, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMOpcode;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Unary opcode
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOpcode {
  FNeg,
  Trunc,
  ZExt,
  SExt,
  FPToUI,
  FPToSI,
  UIToFP,
  SIToFP,
  FPTrunc,
  FPExt,
  PtrToInt,
  IntToPtr,
  BitCast,
}

impl UnaryOpcode {
  pub fn from_llvm(llvm_opcode: LLVMOpcode) -> Option<Self> {
    match llvm_opcode {
      LLVMOpcode::LLVMFNeg => Some(Self::FNeg),
      LLVMOpcode::LLVMTrunc => Some(Self::Trunc),
      LLVMOpcode::LLVMZExt => Some(Self::ZExt),
      LLVMOpcode::LLVMSExt => Some(Self::SExt),
      LLVMOpcode::LLVMFPToUI => Some(Self::FPToUI),
      LLVMOpcode::LLVMFPToSI => Some(Self::FPToSI),
      LLVMOpcode::LLVMUIToFP => Some(Self::UIToFP),
      LLVMOpcode::LLVMSIToFP => Some(Self::SIToFP),
      LLVMOpcode::LLVMFPTrunc => Some(Self::FPTrunc),
      LLVMOpcode::LLVMFPExt => Some(Self::FPExt),
      LLVMOpcode::LLVMPtrToInt => Some(Self::PtrToInt),
      LLVMOpcode::LLVMIntToPtr => Some(Self::IntToPtr),
      LLVMOpcode::LLVMBitCast => Some(Self::BitCast),
      _ => None,
    }
  }

  pub fn to_string(&self) -> &str {
    match self {
      Self::FNeg => "fneg",
      Self::Trunc => "trunc",
      Self::ZExt => "zext",
      Self::SExt => "sext",
      Self::FPToUI => "fptoui",
      Self::FPToSI => "fptosi",
      Self::UIToFP => "uitofp",
      Self::SIToFP => "sitofp",
      Self::FPTrunc => "fptrunc",
      Self::FPExt => "fpext",
      Self::PtrToInt => "ptrtoint",
      Self::IntToPtr => "inttoptr",
      Self::BitCast => "bitcast",
    }
  }
}

/// [Unary instruction](https://llvm.org/docs/LangRef.html#unary-operations)
///
/// Covers the following instruction opcodes:
/// - [Unary](https://llvm.org/docs/LangRef.html#unary-operations)
///   - [FNeg](https://llvm.org/docs/LangRef.html#fneg-instruction)
/// - [Conversion](https://llvm.org/docs/LangRef.html#conversion-operations)
///   - [Trunc](https://llvm.org/docs/LangRef.html#trunc-to-instruction)
///   - [ZExt](https://llvm.org/docs/LangRef.html#zext-to-instruction)
///   - [SExt](https://llvm.org/docs/LangRef.html#sext-to-instruction)
///   - [FPToUI](https://llvm.org/docs/LangRef.html#fptoui-to-instruction)
///   - [FPToSI](https://llvm.org/docs/LangRef.html#fptosi-to-instruction)
///   - [UIToFP](https://llvm.org/docs/LangRef.html#uitofp-to-instruction)
///   - [SIToFP](https://llvm.org/docs/LangRef.html#sitofp-to-instruction)
///   - [FPTrunc](https://llvm.org/docs/LangRef.html#fptrunc-to-instruction)
///   - [FPExt](https://llvm.org/docs/LangRef.html#fpext-to-instruction)
///   - [PtrToInt](https://llvm.org/docs/LangRef.html#ptrtoint-to-instruction)
///   - [IntToPtr](https://llvm.org/docs/LangRef.html#inttoptr-to-instruction)
///   - [BitCast](https://llvm.org/docs/LangRef.html#bitcast-to-instruction)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct UnaryInstruction<'ctx>(UnaryOpcode, LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(UnaryInstruction);

impl_as_operand_for_instr!(UnaryInstruction);

impl_send_sync!(UnaryInstruction);

impl<'ctx> GetType<'ctx> for UnaryInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for UnaryInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for UnaryInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for UnaryInstruction<'ctx> {}

impl<'ctx> UnaryInstruction<'ctx> {
  /// Get the opcode of this unary instruction
  pub fn unary_opcode(&self) -> UnaryOpcode {
    self.0
  }

  /// Get the operand
  pub fn op0(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.1, 0) })
  }
}

impl<'ctx> ValueOpcode for UnaryInstruction<'ctx> {
  fn opcode(&self) -> Opcode {
    Opcode::Unary(self.unary_opcode())
  }
}

impl<'ctx> AsInstruction<'ctx> for UnaryInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Unary(*self)
  }
}

impl_op_from_llvm_value!(UnaryInstruction, UnaryOpcode, LLVMGetInstructionOpcode);

impl_positional_value_ref!(UnaryInstruction, 1);
