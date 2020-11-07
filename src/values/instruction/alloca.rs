use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

/// [Alloca instruction](https://llvm.org/docs/LangRef.html#alloca-instruction)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct AllocaInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(AllocaInstruction);

impl_as_operand_for_instr!(AllocaInstruction);

impl_send_sync!(AllocaInstruction);

impl<'ctx> GetType<'ctx> for AllocaInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for AllocaInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for AllocaInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for AllocaInstruction<'ctx> {}

impl<'ctx> AsInstruction<'ctx> for AllocaInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Alloca(*self)
  }
}

impl<'ctx> ValueOpcode for AllocaInstruction<'ctx> {
  fn opcode(&self) -> Opcode {
    Opcode::Alloca
  }
}

impl<'ctx> AllocaInstruction<'ctx> {
  /// Get the pointer type of alloca
  pub fn get_pointer_type(&self) -> PointerType<'ctx> {
    PointerType::from_llvm(self.get_type().type_ref())
  }

  /// Get the element type which this allocated pointer points to
  pub fn get_element_type(&self) -> Type<'ctx> {
    self.get_pointer_type().element_type()
  }
}

impl_positional_value_ref!(AllocaInstruction, 0);

impl_positional_from_llvm_value!(AllocaInstruction);
