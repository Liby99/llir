use llvm_sys::core::LLVMGetOperand;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// [Select instruction](https://llvm.org/docs/LangRef.html#select-instruction)
///
/// The code `res = a < b ? a : b` will be turned into
///
/// ``` llvm
/// %cmp = icmp slt %a %b
/// %res = select %cmp %a %b
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct SelectInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(SelectInstruction);

impl_as_operand_for_instr!(SelectInstruction);

impl_send_sync!(SelectInstruction);

impl<'ctx> GetType<'ctx> for SelectInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for SelectInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for SelectInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for SelectInstruction<'ctx> {}

impl<'ctx> AsInstruction<'ctx> for SelectInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Select(*self)
  }
}

impl<'ctx> ValueOpcode for SelectInstruction<'ctx> {
  fn opcode(&self) -> Opcode {
    Opcode::Select
  }
}

impl<'ctx> SelectInstruction<'ctx> {
  /// The condition to check
  pub fn condition(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  /// The value to select when the condition is true
  pub fn true_value(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 1) })
  }

  /// The value to select when the condition is false
  pub fn false_value(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 2) })
  }
}

impl_positional_value_ref!(SelectInstruction, 0);

impl_positional_from_llvm_value!(SelectInstruction);
