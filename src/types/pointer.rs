use llvm_sys::core::{LLVMGetElementType};
use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use super::Type;
use crate::{TypeRef, FromLLVMType};

#[derive(Copy, Clone)]
pub struct PointerType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

impl<'ctx> PointerType<'ctx> {
  pub fn element_type(&self) -> Type<'ctx> {
    Type::from_llvm(unsafe { LLVMGetElementType(self.0) })
  }
}

impl<'ctx> TypeRef for PointerType<'ctx> {
  fn type_ref(&self) -> LLVMTypeRef {
    self.0
  }
}

impl<'ctx> FromLLVMType for PointerType<'ctx> {
  fn from_llvm(ptr: LLVMTypeRef) -> Self {
    Self(ptr, PhantomData)
  }
}
