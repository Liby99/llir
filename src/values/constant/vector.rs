use llvm_sys::core::{LLVMGetNumOperands, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

/// [Vector Constant](https://llvm.org/docs/LangRef.html#complex-constants)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VectorConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_send_sync!(VectorConstant);

impl<'ctx> GetType<'ctx> for VectorConstant<'ctx> {}

impl<'ctx> VectorConstant<'ctx> {
  /// Get the number of elements inside this constant vector
  pub fn num_elements(&self) -> usize {
    unsafe { LLVMGetNumOperands(self.0) as usize }
  }

  /// Get the constant elements in a vector
  pub fn elements(&self) -> Vec<Constant<'ctx>> {
    (0..self.num_elements() as u32)
      .map(|i| Constant::from_llvm(unsafe { LLVMGetOperand(self.0, i) }))
      .collect()
  }

  /// Since vector constant definitely has vector type, we have a function to
  /// get directly the vector type
  pub fn get_vector_type(&self) -> VectorType<'ctx> {
    VectorType::from_llvm(self.get_type().type_ref())
  }
}

impl_positional_value_ref!(VectorConstant, 0);

impl_positional_from_llvm_value!(VectorConstant);

impl<'ctx> AsConstant<'ctx> for VectorConstant<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Vector(self.clone())
  }
}

impl_as_operand_for_constant!(VectorConstant);
