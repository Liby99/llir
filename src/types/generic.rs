use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use crate::{FromLLVMType, TypeRef};

/// A placeholder type; used when the type is not supported yet
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GenericType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for GenericType<'ctx> {}

unsafe impl<'ctx> Sync for GenericType<'ctx> {}

impl<'ctx> FromLLVMType for GenericType<'ctx> {
  fn from_llvm(ptr: LLVMTypeRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> TypeRef for GenericType<'ctx> {
  fn type_ref(&self) -> LLVMTypeRef {
    self.0
  }
}
