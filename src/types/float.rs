use llvm_sys::core::LLVMGetTypeKind;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::LLVMTypeKind;
use std::marker::PhantomData;

use crate::types::*;
use crate::{FromLLVMType, TypeRef};

/// The type kind for Float Type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum FloatTypeKind {
  /// 16-bit
  Half,
  /// 32-bit
  Single,
  /// 64-bit
  Double,
  /// 128-bit
  FP128,
  /// 80-bit for X86
  X86_FP80,
  /// 128-bit for Power PC
  PPC_FP128,
}

impl FloatTypeKind {
  /// Get the bit-width of that type kind
  pub fn width(&self) -> u32 {
    match self {
      Self::Half => 16,
      Self::Single => 32,
      Self::Double => 64,
      Self::FP128 => 128,
      Self::X86_FP80 => 80,
      Self::PPC_FP128 => 128,
    }
  }

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

/// [Float Type](https://llvm.org/docs/LangRef.html#floating-point-types)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FloatType<'ctx>(FloatTypeKind, LLVMTypeRef, PhantomData<&'ctx ()>);

impl_send_sync!(FloatType);

impl<'ctx> FloatType<'ctx> {
  /// Get the kind
  pub fn kind(&self) -> FloatTypeKind {
    self.0
  }

  /// Get the bit-width for this float type
  pub fn width(&self) -> u32 {
    self.0.width()
  }
}

impl<'ctx> AsType<'ctx> for FloatType<'ctx> {
  fn as_type(&self) -> Type<'ctx> {
    Type::Float(self.clone())
  }
}

impl_positional_type_ref!(FloatType, 1);

impl<'ctx> FromLLVMType for FloatType<'ctx> {
  fn from_llvm(ptr: LLVMTypeRef) -> Self {
    let kind = FloatTypeKind::from_llvm(unsafe { LLVMGetTypeKind(ptr) }).unwrap();
    Self(kind, ptr, PhantomData)
  }
}
