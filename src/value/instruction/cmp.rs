use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::super::Operand;

#[derive(Copy, Clone)]
pub struct ICmpInstruction<'ctx>(ICmpPredicate, LLVMValueRef, PhantomData<&'ctx ()>);

#[derive(Copy, Clone)]
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

impl<'ctx> ICmpInstruction<'ctx> {
  pub fn predicate(&self) -> ICmpPredicate {
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