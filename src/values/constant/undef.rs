use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// [Undefined value](https://llvm.org/docs/LangRef.html#undefined-values)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Undef<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for Undef<'ctx> {}

unsafe impl<'ctx> Sync for Undef<'ctx> {}

impl<'ctx> GetType<'ctx> for Undef<'ctx> {}

impl<'ctx> ValueRef for Undef<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> FromLLVMValue for Undef<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    // println!("BlockAddr: {} operands", unsafe { LLVMGetNumO})
    Self(ptr, PhantomData)
  }
}

impl<'ctx> AsConstant<'ctx> for Undef<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Undef(self.clone())
  }
}
