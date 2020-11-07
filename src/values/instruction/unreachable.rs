use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// [Unreachable instruction](https://llvm.org/docs/LangRef.html#unreachable-instruction)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct UnreachableInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(UnreachableInstruction);

impl_as_operand_for_instr!(UnreachableInstruction);

impl_send_sync!(UnreachableInstruction);

impl<'ctx> InstructionDebugLoc for UnreachableInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for UnreachableInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for UnreachableInstruction<'ctx> {}

impl<'ctx> ValueOpcode for UnreachableInstruction<'ctx> {
  fn opcode(&self) -> Opcode {
    Opcode::Unreachable
  }
}

impl<'ctx> AsInstruction<'ctx> for UnreachableInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Unreachable(*self)
  }
}

impl_positional_value_ref!(UnreachableInstruction, 0);

impl_positional_from_llvm_value!(UnreachableInstruction);
