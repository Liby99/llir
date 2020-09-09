use llvm_sys::core::{LLVMCountStructElementTypes, LLVMGetOperand, LLVMTypeOf};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

/// [Struct constant](https://llvm.org/docs/LangRef.html#complex-constants)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StructConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_send_sync!(StructConstant);

impl<'ctx> GetType<'ctx> for StructConstant<'ctx> {}

impl<'ctx> StructConstant<'ctx> {
  /// Get the number of elements used to construct the struct
  pub fn num_elements(&self) -> usize {
    unsafe { LLVMCountStructElementTypes(LLVMTypeOf(self.0)) as usize }
  }

  /// Get the elements used to construct the struct
  pub fn elements(&self) -> Vec<Constant<'ctx>> {
    (0..self.num_elements() as u32)
      .map(|i| Constant::from_llvm(unsafe { LLVMGetOperand(self.0, i) }))
      .collect()
  }

  /// Get directly the struct type
  pub fn get_struct_type(&self) -> StructType<'ctx> {
    StructType::from_llvm(self.get_type().type_ref())
  }
}

impl_positional_value_ref!(StructConstant, 0);

impl_positional_from_llvm_value!(StructConstant);

impl<'ctx> AsConstant<'ctx> for StructConstant<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Struct(self.clone())
  }
}

impl_as_operand_for_constant!(StructConstant);
