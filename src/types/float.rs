use llvm_sys::core::LLVMGetTypeKind;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::LLVMTypeKind;
use std::marker::PhantomData;

use crate::{FromLLVMType, TypeRef};

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum FloatTypeKind {
  Half,
  Single,
  Double,
  FP128,
  X86_FP80,
  PPC_FP128,
}

impl FloatTypeKind {
  pub(crate) fn from_llvm(tk: LLVMTypeKind) -> Option<Self> {
    use FloatTypeKind::*;
    use LLVMTypeKind::*;
    match tk {
      LLVMHalfTypeKind => Some(Half),
      LLVMFloatTypeKind => Some(Single),
      LLVMDoubleTypeKind => Some(Double),
      LLVMFP128TypeKind => Some(FP128),
      LLVMX86_FP80TypeKind => Some(X86_FP80),
      LLVMPPC_FP128TypeKind => Some(PPC_FP128),
      _ => None,
    }
  }
}

#[derive(Copy, Clone)]
pub struct FloatType<'ctx>(FloatTypeKind, LLVMTypeRef, PhantomData<&'ctx ()>);

impl<'ctx> FloatType<'ctx> {
  pub fn kind(&self) -> FloatTypeKind {
    self.0
  }
}

impl<'ctx> TypeRef for FloatType<'ctx> {
  fn type_ref(&self) -> LLVMTypeRef {
    self.1
  }
}

impl<'ctx> FromLLVMType for FloatType<'ctx> {
  fn from_llvm(ptr: LLVMTypeRef) -> Self {
    let kind = FloatTypeKind::from_llvm(unsafe { LLVMGetTypeKind(ptr) }).unwrap();
    Self(kind, ptr, PhantomData)
  }
}
