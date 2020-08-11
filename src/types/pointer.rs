use llvm_sys::core::LLVMGetElementType;
use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::{FromLLVMType, TypeRef};

/// [Pointer type](https://llvm.org/docs/LangRef.html#pointer-type)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PointerType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for PointerType<'ctx> {}

unsafe impl<'ctx> Sync for PointerType<'ctx> {}

impl<'ctx> PointerType<'ctx> {
  /// Get the element type of the pointer type
  /// e.g. `"*i32".element_type() == "i32"`
  pub fn element_type(&self) -> Type<'ctx> {
    Type::from_llvm(unsafe { LLVMGetElementType(self.0) })
  }
}

impl<'ctx> AsType<'ctx> for PointerType<'ctx> {
  fn as_type(&self) -> Type<'ctx> {
    Type::Pointer(self.clone())
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
