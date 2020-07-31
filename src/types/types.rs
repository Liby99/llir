use llvm_sys::core::{LLVMGetTypeKind};
use llvm_sys::prelude::{LLVMTypeRef};
use llvm_sys::LLVMTypeKind;
use std::marker::PhantomData;

use super::*;
use crate::{FromLLVMType, TypeRef};

pub enum Type<'ctx> {
  Void(VoidType<'ctx>),
  Int(IntType<'ctx>),
  Float(FloatType<'ctx>),
  Pointer(PointerType<'ctx>),
  Array(ArrayType<'ctx>),
  Other(GenericType<'ctx>),
}

impl<'ctx> FromLLVMType for Type<'ctx> {
  fn from_llvm(ptr: LLVMTypeRef) -> Self {
    use LLVMTypeKind::*;
    match unsafe { LLVMGetTypeKind(ptr) } {
      LLVMVoidTypeKind => Self::Void(VoidType::from_llvm(ptr)),
      LLVMIntegerTypeKind => Self::Int(IntType::from_llvm(ptr)),
      LLVMPointerTypeKind => Self::Pointer(PointerType::from_llvm(ptr)),
      f if FloatTypeKind::from_llvm(f).is_some() => Self::Float(FloatType::from_llvm(ptr)),
      _ => Self::Other(GenericType::from_llvm(ptr)),
    }
  }
}

#[derive(Copy, Clone)]
pub struct GenericType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

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