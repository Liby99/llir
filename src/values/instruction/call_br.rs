use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// [CallBr instruction](https://llvm.org/docs/LangRef.html#callbr-instruction)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct CallBrInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(CallBrInstruction);

impl_as_operand_for_instr!(CallBrInstruction);

impl_send_sync!(CallBrInstruction);

impl<'ctx> GetType<'ctx> for CallBrInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for CallBrInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for CallBrInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for CallBrInstruction<'ctx> {}

impl<'ctx> CallBrInstruction<'ctx> {}

impl<'ctx> ValueOpcode for CallBrInstruction<'ctx> {
  fn opcode(&self) -> Opcode {
    Opcode::CallBr
  }
}

impl<'ctx> AsInstruction<'ctx> for CallBrInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::CallBr(*self)
  }
}

impl_positional_value_ref!(CallBrInstruction, 0);

impl_positional_from_llvm_value!(CallBrInstruction);
