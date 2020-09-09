use llvm_sys::core::{LLVMConstIntGetSExtValue, LLVMConstIntGetZExtValue};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

/// [Integer constant](https://llvm.org/docs/LangRef.html#simple-constants)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IntConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_send_sync!(IntConstant);

impl<'ctx> GetType<'ctx> for IntConstant<'ctx> {}

impl<'ctx> IntConstant<'ctx> {
  /// Zero extended value (u64)
  pub fn zext_value(&self) -> u64 {
    let val = unsafe { LLVMConstIntGetZExtValue(self.0) };
    val as u64
  }

  /// Sign extended value (i64)
  pub fn sext_value(&self) -> i64 {
    let val = unsafe { LLVMConstIntGetSExtValue(self.0) };
    val as i64
  }

  /// Get directly the integer type
  pub fn get_int_type(&self) -> IntType<'ctx> {
    IntType::from_llvm(self.get_type().type_ref())
  }
}

impl_positional_value_ref!(IntConstant, 0);

impl_positional_from_llvm_value!(IntConstant);

impl<'ctx> AsConstant<'ctx> for IntConstant<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Int(self.clone())
  }
}

impl_as_operand_for_constant!(IntConstant);
