use llvm_sys::core::{LLVMGetFCmpPredicate, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMRealPredicate;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Floating point comparison predicate
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FCmpPredicate {
  OEQ,
  OGE,
  OGT,
  OLE,
  OLT,
  ONE,
  ORD,
  PredicateFalse,
  PredicateTrue,
  UEQ,
  UGE,
  UGT,
  ULE,
  ULT,
  UNE,
  UNO,
}

impl FCmpPredicate {
  pub fn from_llvm(pred: LLVMRealPredicate) -> Self {
    use LLVMRealPredicate::*;
    match pred {
      LLVMRealOEQ => Self::OEQ,
      LLVMRealOGE => Self::OGE,
      LLVMRealOGT => Self::OGT,
      LLVMRealOLE => Self::OLE,
      LLVMRealOLT => Self::OLT,
      LLVMRealONE => Self::ONE,
      LLVMRealORD => Self::ORD,
      LLVMRealPredicateFalse => Self::PredicateFalse,
      LLVMRealPredicateTrue => Self::PredicateTrue,
      LLVMRealUEQ => Self::UEQ,
      LLVMRealUGE => Self::UGE,
      LLVMRealUGT => Self::UGT,
      LLVMRealULE => Self::ULE,
      LLVMRealULT => Self::ULT,
      LLVMRealUNE => Self::UNE,
      LLVMRealUNO => Self::UNO,
    }
  }

  pub fn to_string(&self) -> &str {
    match self {
      Self::OEQ => "oeq",
      Self::OGE => "oge",
      Self::OGT => "ogt",
      Self::OLE => "ole",
      Self::OLT => "olt",
      Self::ONE => "one",
      Self::ORD => "ord",
      Self::PredicateFalse => "false",
      Self::PredicateTrue => "true",
      Self::UEQ => "ueq",
      Self::UGE => "uge",
      Self::UGT => "ugt",
      Self::ULE => "ule",
      Self::ULT => "ult",
      Self::UNE => "une",
      Self::UNO => "uno",
    }
  }
}

/// [Floating point comparison (FCmp) instruction](https://llvm.org/docs/LangRef.html#fcmp-instruction)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct FCmpInstruction<'ctx>(FCmpPredicate, LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(FCmpInstruction);

impl_as_operand_for_instr!(FCmpInstruction);

impl_send_sync!(FCmpInstruction);

impl<'ctx> GetType<'ctx> for FCmpInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for FCmpInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for FCmpInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for FCmpInstruction<'ctx> {}

impl<'ctx> FCmpInstruction<'ctx> {
  /// Get the predicate
  pub fn predicate(&self) -> FCmpPredicate {
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

impl<'ctx> ValueOpcode for FCmpInstruction<'ctx> {
  fn opcode(&self) -> Opcode {
    Opcode::FCmp
  }
}

impl<'ctx> AsInstruction<'ctx> for FCmpInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::FCmp(*self)
  }
}

impl_positional_value_ref!(FCmpInstruction, 1);

impl_cmp_from_llvm_value!(FCmpInstruction, FCmpPredicate, LLVMGetFCmpPredicate);
