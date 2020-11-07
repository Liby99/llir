use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// [Load instruction](https://llvm.org/docs/LangRef.html#load-instruction)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct LoadInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(LoadInstruction);

impl_as_operand_for_instr!(LoadInstruction);

impl_send_sync!(LoadInstruction);

impl<'ctx> GetType<'ctx> for LoadInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for LoadInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for LoadInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for LoadInstruction<'ctx> {}

impl<'ctx> LoadInstruction<'ctx> {
  /// Get the location operand which the value is load from
  pub fn location(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }
}

impl<'ctx> ValueOpcode for LoadInstruction<'ctx> {
  fn opcode(&self) -> Opcode {
    Opcode::Load
  }
}

impl<'ctx> AsInstruction<'ctx> for LoadInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Load(*self)
  }
}

impl_positional_value_ref!(LoadInstruction, 0);

impl_positional_from_llvm_value!(LoadInstruction);
