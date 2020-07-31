use llvm_sys::core::{LLVMGetElementType, LLVMGetArrayLength};
use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use super::Type;
use crate::{TypeRef, FromLLVMType};

pub struct ArrayType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

impl<'ctx> ArrayType<'ctx> {
  pub fn element_type(&self) -> Type<'ctx> {
    Type::from_llvm(unsafe { LLVMGetElementType(self.0) })
  }

  pub fn num_elements(&self) -> usize {
    unsafe { LLVMGetArrayLength(self.0) as usize }
  }
}

impl<'ctx> TypeRef for ArrayType<'ctx> {
  fn type_ref(&self) -> LLVMTypeRef {
    self.0
  }
}

impl<'ctx> FromLLVMType for ArrayType<'ctx> {
  fn from_llvm(ptr: LLVMTypeRef) -> Self {
    Self(ptr, PhantomData)
  }
}
