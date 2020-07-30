use llvm_sys::prelude::{LLVMBasicBlockRef, LLVMValueRef};
use std::marker::PhantomData;

use super::instruction::Instruction;

#[derive(Copy, Clone)]
pub struct Block<'ctx>(pub(crate) LLVMBasicBlockRef, pub(crate) PhantomData<&'ctx ()>);

pub struct BlockInstructionIterator<'ctx> {
  curr_instr: Option<LLVMValueRef>,
  marker: PhantomData<&'ctx ()>,
}

impl<'ctx> Iterator for BlockInstructionIterator<'ctx> {
  type Item = Instruction<'ctx>;

  fn next(&mut self) -> Option<Self::Item> {
    // TODO
    None
  }
}