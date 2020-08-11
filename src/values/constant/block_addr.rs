use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Integer constant
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BlockAddressConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for BlockAddressConstant<'ctx> {}

unsafe impl<'ctx> Sync for BlockAddressConstant<'ctx> {}

impl<'ctx> GetType<'ctx> for BlockAddressConstant<'ctx> {}

impl<'ctx> BlockAddressConstant<'ctx> {
  pub fn function(&self) -> Function<'ctx> {
    Function::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  pub fn block(&self) -> Block<'ctx> {
    Block::from_llvm(unsafe { LLVMValueAsBasicBlock(LLVMGetOperand(self.0, 1)) })
  }
}

impl<'ctx> ValueRef for BlockAddressConstant<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> FromLLVMValue for BlockAddressConstant<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    // println!("BlockAddr: {} operands", unsafe { LLVMGetNumO})
    Self(ptr, PhantomData)
  }
}

impl<'ctx> AsConstant<'ctx> for BlockAddressConstant<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::BlockAddress(self.clone())
  }
}
