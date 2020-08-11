use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

/// Inline Assembly value
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct InlineAsm<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> InlineAsm<'ctx> {
  /// Get the type of this InlineAsm in function type form
  pub fn function_type(&self) -> FunctionType<'ctx> {
    FunctionType::from_llvm(unsafe { LLVMGetElementType(LLVMTypeOf(self.0)) })
  }
}

impl<'ctx> AsOperand<'ctx> for InlineAsm<'ctx> {
  fn as_operand(&self) -> Operand<'ctx> {
    Operand::InlineAsm(self.clone())
  }
}

impl<'ctx> FromLLVMValue for InlineAsm<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for InlineAsm<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
