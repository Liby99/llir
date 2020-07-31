use llvm_sys::core::{
  LLVMGetInstructionParent, LLVMGetNextInstruction, LLVMGetNumOperands, LLVMGetOperand, LLVMGetPreviousInstruction,
};

use crate::values::*;
use crate::*;

pub trait AsInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx>;
}

pub trait InstructionTrait<'ctx>: ValueRef {
  fn parent_block(&self) -> Block<'ctx> {
    let value = unsafe { LLVMGetInstructionParent(self.value_ref()) };
    Block::from_llvm(value)
  }

  fn prev_instruction(&self) -> Option<Instruction<'ctx>> {
    let this_ptr = self.value_ref();
    let prev_ptr = unsafe { LLVMGetPreviousInstruction(this_ptr) };
    if prev_ptr.is_null() {
      None
    } else {
      Some(Instruction::from_llvm(prev_ptr))
    }
  }

  fn next_instruction(&self) -> Option<Instruction<'ctx>> {
    let this_ptr = self.value_ref();
    let next_ptr = unsafe { LLVMGetNextInstruction(this_ptr) };
    if next_ptr.is_null() {
      None
    } else {
      Some(Instruction::from_llvm(next_ptr))
    }
  }

  fn num_operands(&self) -> usize {
    unsafe { LLVMGetNumOperands(self.value_ref()) as usize }
  }

  fn operand(&self, index: usize) -> Option<Operand<'ctx>> {
    if index < self.num_operands() {
      Some(Operand::from_llvm(unsafe {
        LLVMGetOperand(self.value_ref(), index as u32)
      }))
    } else {
      None
    }
  }
}
