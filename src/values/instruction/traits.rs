use llvm_sys::core::*;

use crate::values::*;
use crate::*;

/// Turn instructions into Instruction Container Enum
pub trait AsInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx>;
}

pub trait ValueOpcode {
  fn opcode(&self) -> Opcode;
}

pub trait InstructionTrait<'ctx>: AsInstruction<'ctx> + ValueRef {
  /// Get the parent block
  fn parent_block(&self) -> Block<'ctx> {
    let value = unsafe { LLVMGetInstructionParent(self.value_ref()) };
    Block::from_llvm(value)
  }

  /// Get the parent function
  fn parent_function(&self) -> Function<'ctx> {
    self.parent_block().parent_function()
  }

  /// Get the previous instruction of this one
  fn prev_instruction(&self) -> Option<Instruction<'ctx>> {
    let this_ptr = self.value_ref();
    let prev_ptr = unsafe { LLVMGetPreviousInstruction(this_ptr) };
    if prev_ptr.is_null() {
      None
    } else {
      Some(Instruction::from_llvm(prev_ptr))
    }
  }

  /// Get the next instruction of this one
  fn next_instruction(&self) -> Option<Instruction<'ctx>> {
    let this_ptr = self.value_ref();
    let next_ptr = unsafe { LLVMGetNextInstruction(this_ptr) };
    if next_ptr.is_null() {
      None
    } else {
      Some(Instruction::from_llvm(next_ptr))
    }
  }

  /// Get the number of operands used in this instruction
  fn num_operands(&self) -> usize {
    unsafe { LLVMGetNumOperands(self.value_ref()) as usize }
  }

  /// Get the operand at a given index
  fn operand(&self, index: usize) -> Option<Operand<'ctx>> {
    if index < self.num_operands() {
      Some(Operand::from_llvm(unsafe {
        LLVMGetOperand(self.value_ref(), index as u32)
      }))
    } else {
      None
    }
  }

  /// Iterate all the operands of the instruction
  fn iter_operands(&self) -> InstructionOperandIterator<'ctx> {
    InstructionOperandIterator { instr: self.as_instruction(), curr_index: 0 }
  }

  /// Get the string representation of the instruction
  fn to_string(&self) -> String {
    unsafe { utils::raw_to_string(LLVMPrintValueToString(self.value_ref())) }
  }
}

#[doc(hidden)]
pub struct InstructionOperandIterator<'ctx> {
  instr: Instruction<'ctx>,
  curr_index: usize,
}

impl<'ctx> Iterator for InstructionOperandIterator<'ctx> {
  type Item = Operand<'ctx>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.curr_index < self.instr.num_operands() {
      let item = self.instr.operand(self.curr_index);
      self.curr_index += 1;
      Some(item.unwrap())
    } else {
      None
    }
  }
}
