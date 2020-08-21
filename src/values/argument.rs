use llvm_sys::core::LLVMGetParamParent;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Function argument value
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Argument<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for Argument<'ctx> {}

unsafe impl<'ctx> Sync for Argument<'ctx> {}

impl<'ctx> Argument<'ctx> {
  /// Get the parent function that this argument belongs to
  pub fn parent(&self) -> Function<'ctx> {
    Function::from_llvm(unsafe { LLVMGetParamParent(self.0) })
  }

  /// Get the index of this argument wrt the parent function
  pub fn index(&self) -> usize {
    let parent = Function::from_llvm(unsafe { LLVMGetParamParent(self.0) });
    let index = parent.arguments().iter().position(|&a| a.0 == self.0).unwrap();
    index
  }
}

impl<'ctx> FromLLVMValue for Argument<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for Argument<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsOperand<'ctx> for Argument<'ctx> {
  fn as_operand(&self) -> Operand<'ctx> {
    Operand::Argument(self.clone())
  }
}
