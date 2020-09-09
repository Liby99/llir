use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// [Block Address](https://llvm.org/docs/LangRef.html#addresses-of-basic-blocks)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BlockAddress<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_send_sync!(BlockAddress);

impl<'ctx> GetType<'ctx> for BlockAddress<'ctx> {}

impl<'ctx> BlockAddress<'ctx> {
  pub fn function(&self) -> Function<'ctx> {
    Function::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  pub fn block(&self) -> Block<'ctx> {
    Block::from_llvm(unsafe { LLVMValueAsBasicBlock(LLVMGetOperand(self.0, 1)) })
  }
}

impl_positional_value_ref!(BlockAddress, 0);

impl_positional_from_llvm_value!(BlockAddress);

impl<'ctx> AsConstant<'ctx> for BlockAddress<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::BlockAddress(self.clone())
  }
}

impl_as_operand_for_constant!(BlockAddress);
