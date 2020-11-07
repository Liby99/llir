use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

/// [Extract value instruction](https://llvm.org/docs/LangRef.html#extractvalue-instruction)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExtractValueInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(ExtractValueInstruction);

impl_as_operand_for_instr!(ExtractValueInstruction);

impl_send_sync!(ExtractValueInstruction);

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

impl<'ctx> ValueOpcode for ExtractValueInstruction<'ctx> {
  fn opcode(&self) -> Opcode {
    Opcode::ExtractValue
  }
}

impl<'ctx> AsInstruction<'ctx> for ExtractValueInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::ExtractValue(*self)
  }
}

impl_positional_value_ref!(ExtractValueInstruction, 0);

impl_positional_from_llvm_value!(ExtractValueInstruction);
