use llvm_sys::core::{
  LLVMGetElementType, LLVMGetFirstBasicBlock, LLVMGetNextBasicBlock, LLVMIsFunctionVarArg, LLVMTypeOf
};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::utils::string::string_of_value;
use super::Block;
use crate::{FromLLVMBlock, FromLLVMValue, ValueRef};

#[derive(Copy, Clone)]
pub struct Param();

#[derive(Copy, Clone)]
pub struct Function<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> Function<'ctx> {
  pub fn name(&self) -> String {
    string_of_value(self.0)
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
        curr_block: Some(Block::from_llvm(first_block)),
      }
    }
  }

  pub fn is_var_arg(&self) -> bool {
    let functy = unsafe { LLVMGetElementType(LLVMTypeOf(self.0)) };
    let is_var = unsafe { LLVMIsFunctionVarArg(functy) };
    is_var != 0
  }

  pub fn params(&self) -> Vec<Param> {
    // TODO
    vec![]
  }

  pub fn num_params(&self) -> usize {
    // TODO
    0
  }
}

impl<'ctx> ValueRef for Function<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> FromLLVMValue for Function<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
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
        let next_block_ptr = unsafe { LLVMGetNextBasicBlock(block.block_ref()) };
        if next_block_ptr.is_null() {
          self.curr_block = None;
        } else {
          self.curr_block = Some(Block::from_llvm(next_block_ptr));
        }
        result
      }
      None => None,
    }
  }
}
