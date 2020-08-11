use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// [Block Address](https://llvm.org/docs/LangRef.html#addresses-of-basic-blocks)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BlockAddress<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for BlockAddress<'ctx> {}

unsafe impl<'ctx> Sync for BlockAddress<'ctx> {}

impl<'ctx> GetType<'ctx> for BlockAddress<'ctx> {}

impl<'ctx> BlockAddress<'ctx> {
  pub fn function(&self) -> Function<'ctx> {
    Function::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  pub fn block(&self) -> Block<'ctx> {
    Block::from_llvm(unsafe { LLVMValueAsBasicBlock(LLVMGetOperand(self.0, 1)) })
  }
}

impl<'ctx> ValueRef for BlockAddress<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> FromLLVMValue for BlockAddress<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    // println!("BlockAddr: {} operands", unsafe { LLVMGetNumO})
    Self(ptr, PhantomData)
  }
}

impl<'ctx> AsConstant<'ctx> for BlockAddress<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::BlockAddress(self.clone())
  }
}
