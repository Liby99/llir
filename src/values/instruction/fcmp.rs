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
}

/// [Floating point comparison (FCmp) instruction](https://llvm.org/docs/LangRef.html#fcmp-instruction)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct FCmpInstruction<'ctx>(FCmpPredicate, LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(FCmpInstruction);

unsafe impl<'ctx> Send for FCmpInstruction<'ctx> {}

unsafe impl<'ctx> Sync for FCmpInstruction<'ctx> {}

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

impl<'ctx> AsInstruction<'ctx> for FCmpInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::FCmp(*self)
  }
}

impl<'ctx> FromLLVMValue for FCmpInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    let ll_pred = unsafe { LLVMGetFCmpPredicate(ptr) };
    let pred = FCmpPredicate::from_llvm(ll_pred);
    FCmpInstruction(pred, ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for FCmpInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.1
  }
}
