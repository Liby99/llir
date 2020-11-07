use llvm_sys::core::{LLVMGetNumOperands, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

/// [Get Element Pointer (GEP) instruction](https://llvm.org/docs/LangRef.html#getelementptr-instruction)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct GetElementPtrInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(GetElementPtrInstruction);

impl_as_operand_for_instr!(GetElementPtrInstruction);

impl_send_sync!(GetElementPtrInstruction);

impl<'ctx> GetType<'ctx> for GetElementPtrInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for GetElementPtrInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for GetElementPtrInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for GetElementPtrInstruction<'ctx> {}

impl<'ctx> GetElementPtrInstruction<'ctx> {
  /// Get the base location
  pub fn location(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  /// Get the number of indices used to get the element pointer
  pub fn num_indices(&self) -> usize {
    (unsafe { LLVMGetNumOperands(self.0) as usize }) - 1
  }

  /// Get the indices used to get the pointer, in vector form
  pub fn indices(&self) -> Vec<Operand<'ctx>> {
    (0..self.num_indices() as u32)
      .map(|i| Operand::from_llvm(unsafe { LLVMGetOperand(self.0, i + 1) }))
      .collect()
  }

  /// Get the pointer type of the result of this GEP instruction
  pub fn get_pointer_type(&self) -> PointerType<'ctx> {
    PointerType::from_llvm(self.get_type().type_ref())
  }
}

impl<'ctx> ValueOpcode for GetElementPtrInstruction<'ctx> {
  fn opcode(&self) -> Opcode {
    Opcode::GetElementPtr
  }
}

impl<'ctx> AsInstruction<'ctx> for GetElementPtrInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::GetElementPtr(*self)
  }
}

impl_positional_value_ref!(GetElementPtrInstruction, 0);

impl_positional_from_llvm_value!(GetElementPtrInstruction);
