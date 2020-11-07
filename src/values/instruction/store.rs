use llvm_sys::core::LLVMGetOperand;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// [Store instruction](https://llvm.org/docs/LangRef.html#store-instruction)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct StoreInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(StoreInstruction);

impl_as_operand_for_instr!(StoreInstruction);

impl_send_sync!(StoreInstruction);

impl<'ctx> GetDebugMetadata<'ctx> for StoreInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for StoreInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for StoreInstruction<'ctx> {}

impl<'ctx> StoreInstruction<'ctx> {
  /// Get the location operand where the value is stored to
  pub fn location(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 1) })
  }

  /// Get the value operand
  pub fn value(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }
}

impl<'ctx> ValueOpcode for StoreInstruction<'ctx> {
  fn opcode(&self) -> Opcode {
    Opcode::Store
  }
}

impl<'ctx> AsInstruction<'ctx> for StoreInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Store(*self)
  }
}

impl_positional_value_ref!(StoreInstruction, 0);

impl_positional_from_llvm_value!(StoreInstruction);
