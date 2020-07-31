use llvm_sys::core::LLVMGetTypeKind;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::LLVMTypeKind;
use std::marker::PhantomData;

use super::*;
use crate::{FromLLVMType, TypeRef};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Type<'ctx> {
  Void(VoidType<'ctx>),
  Int(IntType<'ctx>),
  Float(FloatType<'ctx>),
  Pointer(PointerType<'ctx>),
  Array(ArrayType<'ctx>),
  Vector(VectorType<'ctx>),
  Struct(StructType<'ctx>),
  Function(FunctionType<'ctx>),
  Other(GenericType<'ctx>),
}

impl<'ctx> FromLLVMType for Type<'ctx> {
  fn from_llvm(ptr: LLVMTypeRef) -> Self {
    use LLVMTypeKind::*;
    match unsafe { LLVMGetTypeKind(ptr) } {
      LLVMVoidTypeKind => Self::Void(VoidType::from_llvm(ptr)),
      LLVMIntegerTypeKind => Self::Int(IntType::from_llvm(ptr)),
      LLVMPointerTypeKind => Self::Pointer(PointerType::from_llvm(ptr)),
      LLVMArrayTypeKind => Self::Array(ArrayType::from_llvm(ptr)),
      LLVMVectorTypeKind => Self::Vector(VectorType::from_llvm(ptr)),
      LLVMStructTypeKind => Self::Struct(StructType::from_llvm(ptr)),
      LLVMFunctionTypeKind => Self::Function(FunctionType::from_llvm(ptr)),
      f if FloatTypeKind::from_llvm(f).is_some() => Self::Float(FloatType::from_llvm(ptr)),
      _ => Self::Other(GenericType::from_llvm(ptr)),
    }
  }
}

impl<'ctx> TypeRef for Type<'ctx> {
  fn type_ref(&self) -> LLVMTypeRef {
    match self {
      Self::Void(v) => v.type_ref(),
      Self::Int(i) => i.type_ref(),
      Self::Float(f) => f.type_ref(),
      Self::Pointer(p) => p.type_ref(),
      Self::Array(a) => a.type_ref(),
      Self::Vector(v) => v.type_ref(),
      Self::Struct(s) => s.type_ref(),
      Self::Function(f) => f.type_ref(),
      Self::Other(o) => o.type_ref(),
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
