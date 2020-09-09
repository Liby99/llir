use llvm_sys::core::LLVMGetParamParent;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// Function argument value
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Argument<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_send_sync!(Argument);

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

impl_positional_value_ref!(Argument, 0);

impl_positional_from_llvm_value!(Argument);

impl<'ctx> AsOperand<'ctx> for Argument<'ctx> {
  fn as_operand(&self) -> Operand<'ctx> {
    Operand::Argument(self.clone())
  }
}
