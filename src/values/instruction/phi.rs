use llvm_sys::core::{LLVMCountIncoming, LLVMGetIncomingBlock, LLVMGetIncomingValue};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Incoming block & value for PHI instruction
pub struct PhiIncoming<'ctx> {
  pub block: Block<'ctx>,
  pub value: Operand<'ctx>,
}

/// [PHI instruction](https://llvm.org/docs/LangRef.html#phi-instruction)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PhiInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_instr_debug!(PhiInstruction);

impl_as_operand_for_instr!(PhiInstruction);

impl_send_sync!(PhiInstruction);

impl<'ctx> GetType<'ctx> for PhiInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for PhiInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for PhiInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for PhiInstruction<'ctx> {}

impl<'ctx> PhiInstruction<'ctx> {
  /// Get the number of incoming nodes
  pub fn num_incomings(&self) -> usize {
    unsafe { LLVMCountIncoming(self.0) as usize }
  }

  /// Get the incomings
  pub fn incomings(&self) -> Vec<PhiIncoming<'ctx>> {
    (0..self.num_incomings())
      .map(|i| PhiIncoming {
        block: Block::from_llvm(unsafe { LLVMGetIncomingBlock(self.0, i as u32) }),
        value: Operand::from_llvm(unsafe { LLVMGetIncomingValue(self.0, i as u32) }),
      })
      .collect()
  }
}

impl<'ctx> ValueOpcode for PhiInstruction<'ctx> {
  fn opcode(&self) -> Opcode {
    Opcode::Phi
  }
}

impl<'ctx> AsInstruction<'ctx> for PhiInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Phi(*self)
  }
}

impl_positional_value_ref!(PhiInstruction, 0);

impl_positional_from_llvm_value!(PhiInstruction);
