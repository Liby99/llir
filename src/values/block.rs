use llvm_sys::core::{
  LLVMGetBasicBlockParent, LLVMGetFirstInstruction, LLVMGetLastInstruction, LLVMGetNextInstruction,
};
use llvm_sys::prelude::{LLVMBasicBlockRef, LLVMValueRef};
use std::marker::PhantomData;

use super::{Function, Instruction};
use crate::{FromLLVMBlock, FromLLVMValue};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Block<'ctx>(LLVMBasicBlockRef, PhantomData<&'ctx ()>);

impl<'ctx> Block<'ctx> {
  pub fn block_ref(&self) -> LLVMBasicBlockRef {
    self.0
  }

  pub fn parent_function(&self) -> Function<'ctx> {
    let func_ptr = unsafe { LLVMGetBasicBlockParent(self.0) };
    Function::from_llvm(func_ptr)
  }

  pub fn iter_instructions(&self) -> BlockInstructionIterator<'ctx> {
    let first_instr = unsafe { LLVMGetFirstInstruction(self.0) };
    if first_instr.is_null() {
      BlockInstructionIterator {
        curr_instr: None,
        marker: PhantomData,
      }
    } else {
      BlockInstructionIterator {
        curr_instr: Some(first_instr),
        marker: PhantomData,
      }
    }
  }

  pub fn first_instruction(&self) -> Option<Instruction<'ctx>> {
    let first_instr = unsafe { LLVMGetFirstInstruction(self.0) };
    if first_instr.is_null() {
      None
    } else {
      Some(Instruction::from_llvm(first_instr))
    }
  }

  pub fn last_instruction(&self) -> Option<Instruction<'ctx>> {
    let last_instr = unsafe { LLVMGetLastInstruction(self.0) };
    if last_instr.is_null() {
      None
    } else {
      Some(Instruction::from_llvm(last_instr))
    }
  }
}

impl<'ctx> FromLLVMBlock for Block<'ctx> {
  fn from_llvm(ptr: LLVMBasicBlockRef) -> Self {
    Block(ptr, PhantomData)
  }
}

pub struct BlockInstructionIterator<'ctx> {
  curr_instr: Option<LLVMValueRef>,
  marker: PhantomData<&'ctx ()>,
}

impl<'ctx> Iterator for BlockInstructionIterator<'ctx> {
  type Item = Instruction<'ctx>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.curr_instr {
      Some(curr_instr_ptr) => {
        let result = Some(Instruction::from_llvm(curr_instr_ptr));
        let next_ptr = unsafe { LLVMGetNextInstruction(curr_instr_ptr) };
        if next_ptr.is_null() {
          self.curr_instr = None;
        } else {
          self.curr_instr = Some(next_ptr);
        }
        result
      }
      None => None,
    }
  }
}
