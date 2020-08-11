use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// CallBr instruction
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExtractValueInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for ExtractValueInstruction<'ctx> {}

unsafe impl<'ctx> Sync for ExtractValueInstruction<'ctx> {}

impl<'ctx> GetType<'ctx> for ExtractValueInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for ExtractValueInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for ExtractValueInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for ExtractValueInstruction<'ctx> {}

impl<'ctx> ExtractValueInstruction<'ctx> {
  pub fn aggregate(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  pub fn indices(&self) -> Vec<IntConstant<'ctx>> {
    let num_operands = unsafe { LLVMGetNumOperands(self.0) as u32 };
    let num_dests = num_operands - 1;
    (1..=num_dests)
      .map(|i| IntConstant::from_llvm(unsafe { LLVMGetOperand(self.0, i) }))
      .collect()
  }
}

impl<'ctx> AsInstruction<'ctx> for ExtractValueInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::ExtractValue(*self)
  }
}

impl<'ctx> FromLLVMValue for ExtractValueInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for ExtractValueInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
