use llvm_sys::core::{LLVMGetElementType, LLVMGetVectorSize};
use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::{FromLLVMType, TypeRef};

/// [Vector type](https://llvm.org/docs/LangRef.html#vector-type)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VectorType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

impl_send_sync!(VectorType);

impl<'ctx> VectorType<'ctx> {
  /// Get the element type inside the vector type
  pub fn element_type(&self) -> Type<'ctx> {
    Type::from_llvm(unsafe { LLVMGetElementType(self.0) })
  }

  /// Get the number of elements in the vector type
  pub fn num_elements(&self) -> usize {
    unsafe { LLVMGetVectorSize(self.0) as usize }
  }
}

impl<'ctx> AsType<'ctx> for VectorType<'ctx> {
  fn as_type(&self) -> Type<'ctx> {
    Type::Vector(self.clone())
  }
}

impl<'ctx> TypeRef for VectorType<'ctx> {
  fn type_ref(&self) -> LLVMTypeRef {
    self.0
  }
}

impl<'ctx> FromLLVMType for VectorType<'ctx> {
  fn from_llvm(ptr: LLVMTypeRef) -> Self {
    Self(ptr, PhantomData)
  }
}
