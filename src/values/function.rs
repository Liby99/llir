use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::utils::string_of_value;
use crate::values::*;
use crate::*;

/// [Function value](https://llvm.org/docs/LangRef.html#functions)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Function<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> std::fmt::Debug for Function<'ctx> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_tuple("Function").field(&self.name()).finish()
  }
}

impl_send_sync!(Function);

impl<'ctx> GetType<'ctx> for Function<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for Function<'ctx> {}

impl<'ctx> Function<'ctx> {
  /// Get the name of the function
  pub fn name(&self) -> String {
    string_of_value(self.0)
  }

  /// Check if this function is declaration only
  pub fn is_declaration_only(&self) -> bool {
    let first_block = unsafe { LLVMGetFirstBasicBlock(self.0) };
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
    let first_block = unsafe { LLVMGetFirstBasicBlock(self.0) };
    if first_block.is_null() {
      FunctionBlockIterator { curr_block: None }
    } else {
      FunctionBlockIterator {
        curr_block: Some(Block::from_llvm(first_block)),
      }
    }
  }

  /// Iterate all the instructions of this function; will iterate the blocks
  /// and then the instructions in each block
  ///
  /// ```
  /// for instr in func.iter_instructions() {
  ///   // Do things with instr...
  /// }
  /// ```
  ///
  /// is exactly the same as
  ///
  /// ```
  /// for blk in func.iter_blocks() {
  ///   for instr in blk.iter_instructions() {
  ///     // Do things with instr...
  ///   }
  /// }
  /// ```
  pub fn iter_instructions(&self) -> FunctionInstructionIterator<'ctx> {
    FunctionInstructionIterator {
      blk_iter: self.iter_blocks(),
      instr_iter: None
    }
  }

  /// Get the number of blocks inside the function
  pub fn num_blocks(&self) -> usize {
    unsafe { LLVMCountBasicBlocks(self.0) as usize }
  }

  /// Get the first block of this function
  pub fn first_block(&self) -> Option<Block<'ctx>> {
    let first_block = unsafe { LLVMGetFirstBasicBlock(self.0) };
    if first_block.is_null() {
      None
    } else {
      Some(Block::from_llvm(first_block))
    }
  }

  /// Get the last block of this function
  pub fn last_block(&self) -> Option<Block<'ctx>> {
    let last_block = unsafe { LLVMGetLastBasicBlock(self.0) };
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

  /// Get the number of arguments in this function
  pub fn num_arguments(&self) -> usize {
    unsafe { LLVMCountParams(self.0) as usize }
  }

  /// Get the arguments to this function in vector form
  pub fn arguments(&self) -> Vec<Argument<'ctx>> {
    (0..self.num_arguments())
      .map(|i| Argument::from_llvm(unsafe { LLVMGetParam(self.0, i as u32) }))
      .collect()
  }

  /// Get the argument at a given index
  pub fn argument(&self, index: usize) -> Argument<'ctx> {
    Argument::from_llvm(unsafe { LLVMGetParam(self.0, index as u32) })
  }

  /// Get the function type
  pub fn get_function_type(&self) -> FunctionType<'ctx> {
    let type_ref = unsafe { LLVMGetElementType(LLVMTypeOf(self.0)) };
    FunctionType::from_llvm(type_ref)
  }
}

impl_positional_value_ref!(Function, 0);

impl_positional_from_llvm_value!(Function);

impl<'ctx> AsConstant<'ctx> for Function<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Function(self.clone())
  }
}

impl_as_operand_for_constant!(Function);

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

#[doc(hidden)]
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

#[doc(hidden)]
pub struct FunctionInstructionIterator<'ctx> {
  blk_iter: FunctionBlockIterator<'ctx>,
  instr_iter: Option<BlockInstructionIterator<'ctx>>,
}

impl<'ctx> Iterator for FunctionInstructionIterator<'ctx> {
  type Item = Instruction<'ctx>;

  fn next(&mut self) -> Option<Self::Item> {
    match &mut self.instr_iter {
      Some(instr_iter) => match instr_iter.next() {
        Some(next_instr) => return Some(next_instr),
        None => {}
      }
      None => {}
    };
    match self.blk_iter.next() {
      Some(blk) => {
        let mut instr_iter = blk.iter_instructions();
        let result = instr_iter.next();
        self.instr_iter = Some(instr_iter);
        result
      }
      None => None
    }
  }
}