use llvm_sys::core::LLVMGetParamParent;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Function argument value
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Argument<'ctx>(usize, LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for Argument<'ctx> {}

unsafe impl<'ctx> Sync for Argument<'ctx> {}

impl<'ctx> Argument<'ctx> {
  /// Get the parent function that this argument belongs to
  pub fn parent(&self) -> Function<'ctx> {
    Function::from_llvm(unsafe { LLVMGetParamParent(self.1) })
  }

  /// Get the index of this argument wrt the parent function
  pub fn index(&self) -> usize {
    self.0
  }
}

impl<'ctx> FromLLVMValue for Argument<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    let parent = Function::from_llvm(unsafe { LLVMGetParamParent(ptr) });
    let index = parent.arguments().iter().position(|&a| a.1 == ptr).unwrap();
    Self(index, ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for Argument<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.1
  }
}
