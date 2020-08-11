use llvm_sys::core::{LLVMGetNumOperands, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

/// Vector Constant
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VectorConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for VectorConstant<'ctx> {}

unsafe impl<'ctx> Sync for VectorConstant<'ctx> {}

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

impl<'ctx> FromLLVMValue for VectorConstant<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for VectorConstant<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsConstant<'ctx> for VectorConstant<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Vector(self.clone())
  }
}
