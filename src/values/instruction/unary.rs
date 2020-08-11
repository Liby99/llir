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
}

/// Unary instruction
///
/// Covers the following instruction opcodes:
/// - FNeg
/// - Trunc
/// - ZExt
/// - SExt
/// - FPToUI
/// - FPToSI
/// - UIToFP
/// - SIToFP
/// - FPTrunc
/// - FPExt
/// - PtrToInt
/// - IntToPtr
/// - BitCast
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct UnaryInstruction<'ctx>(UnaryOpcode, LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for UnaryInstruction<'ctx> {}

unsafe impl<'ctx> Sync for UnaryInstruction<'ctx> {}

impl<'ctx> GetType<'ctx> for UnaryInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for UnaryInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for UnaryInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for UnaryInstruction<'ctx> {}

impl<'ctx> UnaryInstruction<'ctx> {
  /// Get the opcode of this unary instruction
  pub fn opcode(&self) -> UnaryOpcode {
    self.0
  }

  /// Get the operand
  pub fn op0(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.1, 0) })
  }
}

impl<'ctx> AsInstruction<'ctx> for UnaryInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Unary(*self)
  }
}

impl<'ctx> FromLLVMValue for UnaryInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    let bin_op = UnaryOpcode::from_llvm(unsafe { LLVMGetInstructionOpcode(ptr) }).unwrap();
    UnaryInstruction(bin_op, ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for UnaryInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.1
  }
}
