use llvm_sys::core::LLVMGetIntTypeWidth;
use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use crate::{FromLLVMType, TypeRef};

#[derive(Copy, Clone)]
pub struct IntType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

impl<'ctx> IntType<'ctx> {
  pub fn bits(&self) -> u32 {
    unsafe { LLVMGetIntTypeWidth(self.0) }
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
