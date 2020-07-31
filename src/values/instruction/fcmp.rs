use llvm_sys::core::{LLVMGetFCmpPredicate, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMRealPredicate;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

#[derive(Debug, Copy, Clone)]
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
    match pred {
      LLVMRealPredicate::LLVMRealOEQ => Self::OEQ,
      LLVMRealPredicate::LLVMRealOGE => Self::OGE,
      LLVMRealPredicate::LLVMRealOGT => Self::OGT,
      LLVMRealPredicate::LLVMRealOLE => Self::OLE,
      LLVMRealPredicate::LLVMRealOLT => Self::OLT,
      LLVMRealPredicate::LLVMRealONE => Self::ONE,
      LLVMRealPredicate::LLVMRealORD => Self::ORD,
      LLVMRealPredicate::LLVMRealPredicateFalse => Self::PredicateFalse,
      LLVMRealPredicate::LLVMRealPredicateTrue => Self::PredicateTrue,
      LLVMRealPredicate::LLVMRealUEQ => Self::UEQ,
      LLVMRealPredicate::LLVMRealUGE => Self::UGE,
      LLVMRealPredicate::LLVMRealUGT => Self::UGT,
      LLVMRealPredicate::LLVMRealULE => Self::ULE,
      LLVMRealPredicate::LLVMRealULT => Self::ULT,
      LLVMRealPredicate::LLVMRealUNE => Self::UNE,
      LLVMRealPredicate::LLVMRealUNO => Self::UNO,
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct FCmpInstruction<'ctx>(FCmpPredicate, LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> HasType for FCmpInstruction<'ctx> {}

impl<'ctx> FCmpInstruction<'ctx> {
  pub fn predicate(&self) -> FCmpPredicate {
    self.0
  }

  pub fn op0(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.1, 0) })
  }

  pub fn op1(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.1, 1) })
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
