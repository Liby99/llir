use llvm_sys::core::LLVMGetIntTypeWidth;
use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::{FromLLVMType, TypeRef};

/// [Integer type](https://llvm.org/docs/LangRef.html#integer-type)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IntType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for IntType<'ctx> {}

unsafe impl<'ctx> Sync for IntType<'ctx> {}

impl<'ctx> IntType<'ctx> {
  /// Get the number of bits
  pub fn bits(&self) -> u32 {
    unsafe { LLVMGetIntTypeWidth(self.0) }
  }
}

impl<'ctx> AsType<'ctx> for IntType<'ctx> {
  fn as_type(&self) -> Type<'ctx> {
    Type::Int(self.clone())
  }
}

impl<'ctx> TypeRef for IntType<'ctx> {
  fn type_ref(&self) -> LLVMTypeRef {
    self.0
  }
}

impl<'ctx> FromLLVMType for IntType<'ctx> {
  fn from_llvm(ptr: LLVMTypeRef) -> Self {
    Self(ptr, PhantomData)
  }
}
