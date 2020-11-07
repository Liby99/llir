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

impl_send_sync!(ReturnInstruction);

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

impl<'ctx> ValueOpcode for ReturnInstruction<'ctx> {
  fn opcode(&self) -> Opcode {
    Opcode::Ret
  }
}

impl<'ctx> AsInstruction<'ctx> for ReturnInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Return(*self)
  }
}

impl_positional_value_ref!(ReturnInstruction, 0);

impl_positional_from_llvm_value!(ReturnInstruction);
