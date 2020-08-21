use llvm_sys::core::{LLVMGetNumOperands, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// [Return (Ret) instruction](https://llvm.org/docs/LangRef.html#ret-instruction)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ReturnInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(ReturnInstruction);

impl_as_operand_for_instr!(ReturnInstruction);

unsafe impl<'ctx> Send for ReturnInstruction<'ctx> {}

unsafe impl<'ctx> Sync for ReturnInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for ReturnInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for ReturnInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for ReturnInstruction<'ctx> {}

impl<'ctx> ReturnInstruction<'ctx> {
  /// Check if this return instruction has returned value
  pub fn has_op(&self) -> bool {
    unsafe { LLVMGetNumOperands(self.0) != 0 }
  }

  /// Get the returned value. The instruction might not contain one
  pub fn op(&self) -> Option<Operand<'ctx>> {
    if self.has_op() {
      Some(Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) }))
    } else {
      None
    }
  }
}

impl<'ctx> AsInstruction<'ctx> for ReturnInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Return(*self)
  }
}

impl<'ctx> FromLLVMValue for ReturnInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for ReturnInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
