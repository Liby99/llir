use llvm_sys::core::{LLVMGetArrayLength, LLVMGetOperand, LLVMTypeOf};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::Constant;
use crate::types::*;
use crate::values::*;
use crate::*;

/// [Array constant](https://llvm.org/docs/LangRef.html#complex-constants)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ArrayConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_send_sync!(ArrayConstant);

impl<'ctx> GetType<'ctx> for ArrayConstant<'ctx> {}

impl<'ctx> ArrayConstant<'ctx> {
  /// Get the number of elements used to construct the array constant
  pub fn num_elements(&self) -> usize {
    unsafe { LLVMGetArrayLength(LLVMTypeOf(self.0)) as usize }
  }

  /// Get the elements used to construct the array constant
  pub fn elements(&self) -> Vec<Constant<'ctx>> {
    (0..self.num_elements() as u32)
      .map(|i| Constant::from_llvm(unsafe { LLVMGetOperand(self.0, i) }))
      .collect()
  }

  /// Get directly the array type
  pub fn get_array_type(&self) -> ArrayType<'ctx> {
    ArrayType::from_llvm(self.get_type().type_ref())
  }
}

impl_positional_value_ref!(ArrayConstant, 0);

impl_positional_from_llvm_value!(ArrayConstant);

impl<'ctx> AsConstant<'ctx> for ArrayConstant<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Array(self.clone())
  }
}

impl_as_operand_for_constant!(ArrayConstant);