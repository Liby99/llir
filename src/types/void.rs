use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use crate::{FromLLVMType, TypeRef};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VoidType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

impl<'ctx> TypeRef for VoidType<'ctx> {
  fn type_ref(&self) -> LLVMTypeRef {
    self.0
  }
}

impl<'ctx> FromLLVMType for VoidType<'ctx> {
  fn from_llvm(ptr: LLVMTypeRef) -> Self {
    Self(ptr, PhantomData)
  }
}
