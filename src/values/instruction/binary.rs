use llvm_sys::core::{LLVMGetInstructionOpcode, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMOpcode;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Binary Opcode
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOpcode {
  // Arithmatics
  Add,
  Sub,
  Mul,
  UDiv,
  SDiv,
  URem,
  SRem,
  // Floating point
  FAdd,
  FSub,
  FMul,
  FDiv,
  FRem,
  // Bitwise operation
  Shl,
  LShr,
  AShr,
  And,
  Or,
  Xor,
}

impl BinaryOpcode {
  pub fn from_llvm(llvm_opcode: LLVMOpcode) -> Option<Self> {
    match llvm_opcode {
      LLVMOpcode::LLVMAdd => Some(Self::Add),
      LLVMOpcode::LLVMSub => Some(Self::Sub),
      LLVMOpcode::LLVMMul => Some(Self::Mul),
      LLVMOpcode::LLVMUDiv => Some(Self::UDiv),
      LLVMOpcode::LLVMSDiv => Some(Self::SDiv),
      LLVMOpcode::LLVMURem => Some(Self::URem),
      LLVMOpcode::LLVMSRem => Some(Self::SRem),
      LLVMOpcode::LLVMFAdd => Some(Self::FAdd),
      LLVMOpcode::LLVMFSub => Some(Self::FSub),
      LLVMOpcode::LLVMFMul => Some(Self::FMul),
      LLVMOpcode::LLVMFDiv => Some(Self::FDiv),
      LLVMOpcode::LLVMShl => Some(Self::Shl),
      LLVMOpcode::LLVMLShr => Some(Self::LShr),
      LLVMOpcode::LLVMAShr => Some(Self::AShr),
      LLVMOpcode::LLVMAnd => Some(Self::And),
      LLVMOpcode::LLVMOr => Some(Self::Or),
      LLVMOpcode::LLVMXor => Some(Self::Xor),
      _ => None,
    }
  }
}

/// [Binary instruction](https://llvm.org/docs/LangRef.html#binary-operations)
///
/// Covers the following instruction opcodes:
/// - Integer Arithmetics
///   - Add
///   - Sub
///   - Mul
///   - UDiv
///   - SDiv
///   - URem
///   - SRem
/// - Floating Point Arithmetics
///   - FAdd
///   - FSub
///   - FMul
///   - FDiv
/// - Bitwise
///   - Shl
///   - LShr
///   - AShr
///   - And
///   - Or
///   - Xor
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct BinaryInstruction<'ctx>(BinaryOpcode, LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(BinaryInstruction);

unsafe impl<'ctx> Send for BinaryInstruction<'ctx> {}

unsafe impl<'ctx> Sync for BinaryInstruction<'ctx> {}

impl<'ctx> GetType<'ctx> for BinaryInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for BinaryInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for BinaryInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for BinaryInstruction<'ctx> {}

impl<'ctx> AsInstruction<'ctx> for BinaryInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Binary(*self)
  }
}

impl<'ctx> BinaryInstruction<'ctx> {
  /// Get the opcode of this binary instruction
  pub fn opcode(&self) -> BinaryOpcode {
    self.0
  }

  /// Get the lhs operand
  pub fn op0(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.1, 0) })
  }

  /// Get the rhs operand
  pub fn op1(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.1, 1) })
  }
}

impl<'ctx> FromLLVMValue for BinaryInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    let bin_op = BinaryOpcode::from_llvm(unsafe { LLVMGetInstructionOpcode(ptr) }).unwrap();
    BinaryInstruction(bin_op, ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for BinaryInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.1
  }
}
