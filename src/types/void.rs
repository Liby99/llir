use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::{FromLLVMType, TypeRef};

/// Void type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VoidType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for VoidType<'ctx> {}

unsafe impl<'ctx> Sync for VoidType<'ctx> {}

impl<'ctx> AsType<'ctx> for VoidType<'ctx> {
  fn as_type(&self) -> Type<'ctx> {
    Type::Void(self.clone())
  }
}

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
