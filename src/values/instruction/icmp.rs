use llvm_sys::core::{LLVMGetICmpPredicate, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMIntPredicate;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Integer conparison predicate
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ICmpPredicate {
  /// Equals
  EQ,
  /// Not-equals
  NE,
  /// Signed greater-than or equal-to
  SGE,
  /// Unsigned greater-than or equal-to
  UGE,
  /// Signed greater-than
  SGT,
  /// Unsigned greater-than
  UGT,
  /// Signed less-than or equal-to
  SLE,
  /// Unsigned less-than or equal-to
  ULE,
  /// Signed less-than
  SLT,
  /// Unsigned less-than
  ULT,
}

impl ICmpPredicate {
  pub(crate) fn from_llvm(pred: LLVMIntPredicate) -> Self {
    use LLVMIntPredicate::*;
    match pred {
      LLVMIntEQ => Self::EQ,
      LLVMIntNE => Self::NE,
      LLVMIntSGE => Self::SGE,
      LLVMIntUGE => Self::UGE,
      LLVMIntSGT => Self::SGT,
      LLVMIntUGT => Self::UGT,
      LLVMIntSLE => Self::SLE,
      LLVMIntULE => Self::ULE,
      LLVMIntSLT => Self::SLT,
      LLVMIntULT => Self::ULT,
    }
  }
}

/// [Integer comparison (ICmp) instruction](https://llvm.org/docs/LangRef.html#icmp-instruction)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ICmpInstruction<'ctx>(ICmpPredicate, LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(ICmpInstruction);

impl_as_operand_for_instr!(ICmpInstruction);

impl_send_sync!(ICmpInstruction);

impl<'ctx> GetType<'ctx> for ICmpInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for ICmpInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for ICmpInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for ICmpInstruction<'ctx> {}

impl<'ctx> ICmpInstruction<'ctx> {
  /// Get the integer comparison predicate
  pub fn predicate(&self) -> ICmpPredicate {
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

impl<'ctx> AsInstruction<'ctx> for ICmpInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::ICmp(*self)
  }
}

impl_positional_value_ref!(ICmpInstruction, 1);

impl_cmp_from_llvm_value!(ICmpInstruction, ICmpPredicate, LLVMGetICmpPredicate);
