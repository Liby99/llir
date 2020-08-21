use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

/// [Null constant](https://llvm.org/docs/LangRef.html#simple-constants)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NullConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for NullConstant<'ctx> {}

unsafe impl<'ctx> Sync for NullConstant<'ctx> {}

impl<'ctx> GetType<'ctx> for NullConstant<'ctx> {}

impl<'ctx> NullConstant<'ctx> {
  /// Get directly the pointer type of this null constant
  pub fn get_pointer_type(&self) -> PointerType<'ctx> {
    PointerType::from_llvm(self.get_type().type_ref())
  }
}

impl<'ctx> ValueRef for NullConstant<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> FromLLVMValue for NullConstant<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> AsConstant<'ctx> for NullConstant<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Null(self.clone())
  }
}

impl_as_operand_for_constant!(NullConstant);
