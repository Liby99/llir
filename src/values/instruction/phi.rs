use llvm_sys::core::{LLVMCountIncoming, LLVMGetIncomingBlock, LLVMGetIncomingValue};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

pub struct Incoming<'ctx> {
  pub block: Block<'ctx>,
  pub value: Operand<'ctx>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PhiInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> HasType for PhiInstruction<'ctx> {}

impl<'ctx> PhiInstruction<'ctx> {
  pub fn num_incomings(&self) -> usize {
    unsafe { LLVMCountIncoming(self.0) as usize }
  }

  pub fn incomings(&self) -> Vec<Incoming<'ctx>> {
    (0..self.num_incomings())
      .map(|i| Incoming {
        block: Block::from_llvm(unsafe { LLVMGetIncomingBlock(self.0, i as u32) }),
        value: Operand::from_llvm(unsafe { LLVMGetIncomingValue(self.0, i as u32) }),
      })
      .collect()
  }
}

impl<'ctx> FromLLVMValue for PhiInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for PhiInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
