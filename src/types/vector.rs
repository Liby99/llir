use llvm_sys::core::{LLVMGetElementType, LLVMGetVectorSize};
use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use super::Type;
use crate::{FromLLVMType, TypeRef};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VectorType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

impl<'ctx> VectorType<'ctx> {
  pub fn element_type(&self) -> Type<'ctx> {
    Type::from_llvm(unsafe { LLVMGetElementType(self.0) })
  }

  pub fn num_elements(&self) -> usize {
    unsafe { LLVMGetVectorSize(self.0) as usize }
  }
}

impl<'ctx> TypeRef for VectorType<'ctx> {
  fn type_ref(&self) -> LLVMTypeRef {
    self.0
  }
}

impl<'ctx> FromLLVMType for VectorType<'ctx> {
  fn from_llvm(ptr: LLVMTypeRef) -> Self {
    Self(ptr, PhantomData)
  }
}
