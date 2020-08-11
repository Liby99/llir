use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
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
  /// Get the aggregate operand
  pub fn aggregate(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  /// Get the aggregate type
  pub fn aggregate_type(&self) -> Type<'ctx> {
    self.aggregate().get_type()
  }

  /// Get the number of indices
  pub fn num_indices(&self) -> usize {
    unsafe { LLVMGetNumIndices(self.0) as usize }
  }

  /// Get the indices
  pub fn indices(&self) -> Vec<u32> {
    let num_indices = self.num_indices();
    let mut indices = vec![0; num_indices];
    unsafe {
      let raw_indices = LLVMGetIndices(self.0);
      for i in 0..num_indices {
        indices[i] = *raw_indices.offset(i as isize) as u32;
      }
    }
    return indices;
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
