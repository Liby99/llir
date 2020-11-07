use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

/// [Insert value instruction](https://llvm.org/docs/LangRef.html#insertvalue-instruction)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct InsertValueInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(InsertValueInstruction);

impl_as_operand_for_instr!(InsertValueInstruction);

impl_send_sync!(InsertValueInstruction);

impl<'ctx> GetType<'ctx> for InsertValueInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for InsertValueInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for InsertValueInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for InsertValueInstruction<'ctx> {}

impl<'ctx> InsertValueInstruction<'ctx> {
  /// Get the aggregate operand
  pub fn aggregate(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  /// Get the aggregate type
  pub fn aggregate_type(&self) -> Type<'ctx> {
    self.aggregate().get_type()
  }

  /// Get the inserted value
  pub fn value(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 1) })
  }

  /// Get the inserted value type
  pub fn value_type(&self) -> Type<'ctx> {
    self.value().get_type()
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

impl<'ctx> ValueOpcode for InsertValueInstruction<'ctx> {
  fn opcode(&self) -> Opcode {
    Opcode::InsertValue
  }
}

impl<'ctx> AsInstruction<'ctx> for InsertValueInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::InsertValue(*self)
  }
}

impl_positional_value_ref!(InsertValueInstruction, 0);

impl_positional_from_llvm_value!(InsertValueInstruction);
