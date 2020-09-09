use llvm_sys::core::LLVMGetIntTypeWidth;
use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::{FromLLVMType, TypeRef};

/// [Integer type](https://llvm.org/docs/LangRef.html#integer-type)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IntType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

impl_send_sync!(IntType);

impl<'ctx> IntType<'ctx> {
  /// Get the bit-width
  pub fn width(&self) -> u32 {
    unsafe { LLVMGetIntTypeWidth(self.0) }
  }
}

impl<'ctx> AsType<'ctx> for IntType<'ctx> {
  fn as_type(&self) -> Type<'ctx> {
    Type::Int(self.clone())
  }
}

impl_positional_type_ref!(IntType, 0);

impl_positional_from_llvm_type!(IntType);