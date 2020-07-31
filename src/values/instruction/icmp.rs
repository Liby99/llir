use llvm_sys::core::{LLVMGetICmpPredicate, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::{LLVMIntPredicate};
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

#[derive(Debug, Copy, Clone)]
pub enum ICmpPredicate {
  EQ,
  NE,
  SGE,
  UGE,
  SGT,
  UGT,
  SLE,
  ULE,
  SLT,
  ULT,
}

impl ICmpPredicate {
  pub fn from_llvm(pred: LLVMIntPredicate) -> Self {
    match pred {
      LLVMIntPredicate::LLVMIntEQ => Self::EQ,
      LLVMIntPredicate::LLVMIntNE => Self::NE,
      LLVMIntPredicate::LLVMIntSGE => Self::SGE,
      LLVMIntPredicate::LLVMIntUGE => Self::UGE,
      LLVMIntPredicate::LLVMIntSGT => Self::SGT,
      LLVMIntPredicate::LLVMIntUGT => Self::UGT,
      LLVMIntPredicate::LLVMIntSLE => Self::SLE,
      LLVMIntPredicate::LLVMIntULE => Self::ULE,
      LLVMIntPredicate::LLVMIntSLT => Self::SLT,
      LLVMIntPredicate::LLVMIntULT => Self::ULT,
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct ICmpInstruction<'ctx>(ICmpPredicate, LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> HasType for ICmpInstruction<'ctx> {}

impl<'ctx> ICmpInstruction<'ctx> {
  pub fn predicate(&self) -> ICmpPredicate {
    self.0
  }

  pub fn op0(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.1, 0) })
  }

  pub fn op1(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.1, 1) })
  }
}

impl<'ctx> FromLLVMValue for ICmpInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    let ll_pred = unsafe { LLVMGetICmpPredicate(ptr) };
    let pred = ICmpPredicate::from_llvm(ll_pred);
    ICmpInstruction(pred, ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for ICmpInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.1
  }
}
