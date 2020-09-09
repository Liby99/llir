use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::{FromLLVMType, TypeRef};

/// [Void type](https://llvm.org/docs/LangRef.html#void-type)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VoidType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

impl_send_sync!(VoidType);

impl<'ctx> AsType<'ctx> for VoidType<'ctx> {
  fn as_type(&self) -> Type<'ctx> {
    Type::Void(self.clone())
  }
}

impl_positional_type_ref!(VoidType, 0);

impl_positional_from_llvm_type!(VoidType);