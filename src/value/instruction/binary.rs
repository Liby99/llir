use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMOpcode;
use std::marker::PhantomData;

use super::super::{Operand, ValueRef};

#[derive(Copy, Clone)]
pub struct BinaryInstruction<'ctx>(BinaryOpcode, LLVMValueRef, PhantomData<&'ctx ()>);

#[derive(Copy, Clone)]
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

impl<'ctx> BinaryInstruction<'ctx> {
  pub fn opcode(&self) -> BinaryOpcode {
    self.0
  }

  pub fn op0(&self) -> Operand<'ctx> {
    // TODO
    Operand::Metadata
  }

  pub fn op1(&self) -> Operand<'ctx> {
    // TODO
    Operand::Metadata
  }
}

impl<'ctx> ValueRef for BinaryInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.1
  }
}
