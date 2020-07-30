use llvm_sys::core::{LLVMGetFirstBasicBlock, LLVMGetNextBasicBlock};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::block::Block;

#[derive(Copy, Clone)]
pub struct Function<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> Function<'ctx> {
  pub fn from_llvm(_ptr: LLVMValueRef) -> Option<Self> {
    // TODO
    None
  }

  pub fn is_declaration_only(&self) -> bool {
    let first_block = unsafe { LLVMGetFirstBasicBlock(self.0) };
    first_block.is_null()
  }

  pub fn iter_blocks(&self) -> FunctionBlockIterator<'ctx> {
    let first_block = unsafe { LLVMGetFirstBasicBlock(self.0) };
    if first_block.is_null() {
      FunctionBlockIterator { curr_block: None }
    } else {
      FunctionBlockIterator {
        curr_block: Some(Block(first_block, PhantomData)),
      }
    }
  }
}

pub struct FunctionBlockIterator<'ctx> {
  curr_block: Option<Block<'ctx>>,
}

impl<'ctx> Iterator for FunctionBlockIterator<'ctx> {
  type Item = Block<'ctx>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.curr_block {
      Some(block) => {
        let result = Some(block);
        let next_block_ptr = unsafe { LLVMGetNextBasicBlock(block.0) };
        if next_block_ptr.is_null() {
          self.curr_block = None;
        } else {
          self.curr_block = Some(Block(next_block_ptr, PhantomData));
        }
        result
      }
      None => None,
    }
  }
}
