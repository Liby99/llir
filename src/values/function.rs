use llvm_sys::core::{
  LLVMCountParams, LLVMGetElementType, LLVMGetFirstBasicBlock, LLVMGetLastBasicBlock, LLVMGetNextBasicBlock,
  LLVMGetParam, LLVMTypeOf,
};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::utils::string_of_value;
use crate::values::*;
use crate::*;

/// Function value
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Function<'ctx> {
  func: LLVMValueRef,
  marker: PhantomData<&'ctx ()>,
}

impl<'ctx> GetType<'ctx> for Function<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for Function<'ctx> {}

impl<'ctx> Function<'ctx> {
  /// Get the name of the function
  pub fn name(&self) -> String {
    string_of_value(self.func)
  }

  /// Check if this function is declaration only
  pub fn is_declaration_only(&self) -> bool {
    let first_block = unsafe { LLVMGetFirstBasicBlock(self.func) };
    first_block.is_null()
  }

  /// Iterate blocks inside of this function
  ///
  /// ```
  /// for blk in func.iter_blocks() {
  ///   // Do things to blk...
  /// }
  /// ```
  pub fn iter_blocks(&self) -> FunctionBlockIterator<'ctx> {
    let first_block = unsafe { LLVMGetFirstBasicBlock(self.func) };
    if first_block.is_null() {
      FunctionBlockIterator { curr_block: None }
    } else {
      FunctionBlockIterator {
        curr_block: Some(Block::from_llvm(first_block)),
      }
    }
  }

  /// Get the first block of this function
  pub fn first_block(&self) -> Option<Block<'ctx>> {
    let first_block = unsafe { LLVMGetFirstBasicBlock(self.func) };
    if first_block.is_null() {
      None
    } else {
      Some(Block::from_llvm(first_block))
    }
  }

  /// Get the last block of this function
  pub fn last_block(&self) -> Option<Block<'ctx>> {
    let last_block = unsafe { LLVMGetLastBasicBlock(self.func) };
    if last_block.is_null() {
      None
    } else {
      Some(Block::from_llvm(last_block))
    }
  }

  /// Check if the function is variable arguments
  pub fn is_var_arg(&self) -> bool {
    self.get_function_type().is_var_arg()
  }

  /// Get the arguments to this function in vector form
  pub fn arguments(&self) -> Vec<Argument<'ctx>> {
    (0..self.num_arguments())
      .map(|i| Argument::from_llvm(unsafe { LLVMGetParam(self.func, i as u32) }))
      .collect()
  }

  /// Get the number of arguments in this function
  pub fn num_arguments(&self) -> usize {
    unsafe { LLVMCountParams(self.func) as usize }
  }

  /// Get the function type
  pub fn get_function_type(&self) -> FunctionType<'ctx> {
    let type_ref = unsafe { LLVMGetElementType(LLVMTypeOf(self.func)) };
    FunctionType::from_llvm(type_ref)
  }
}

impl<'ctx> ValueRef for Function<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.func
  }
}

impl<'ctx> FromLLVMValue for Function<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self {
      func: ptr,
      marker: PhantomData,
    }
  }
}

impl<'ctx> GetDebugLoc for Function<'ctx> {
  fn filename(&self) -> Option<String> {
    match self.first_block() {
      Some(block) => {
        for instr in block.iter_instructions() {
          match instr.filename() {
            Some(filename) => return Some(filename),
            _ => {}
          }
        }
        None
      }
      _ => None,
    }
  }

  fn line(&self) -> Option<u32> {
    None
  }

  fn col(&self) -> Option<u32> {
    None
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
