use llvm_sys::core::{LLVMGetArrayLength, LLVMGetElementType};
use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::{FromLLVMType, TypeRef};

/// [Array Type](https://llvm.org/docs/LangRef.html#array-type)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for ArrayType<'ctx> {}

unsafe impl<'ctx> Sync for ArrayType<'ctx> {}

impl<'ctx> ArrayType<'ctx> {
  /// Get the element_type of the Array Type
  pub fn element_type(&self) -> Type<'ctx> {
    Type::from_llvm(unsafe { LLVMGetElementType(self.0) })
  }

  /// Get the number of elements in the Array Type
  pub fn num_elements(&self) -> usize {
    unsafe { LLVMGetArrayLength(self.0) as usize }
  }
}

impl<'ctx> AsType<'ctx> for ArrayType<'ctx> {
  fn as_type(&self) -> Type<'ctx> {
    Type::Array(self.clone())
  }
}

impl<'ctx> TypeRef for ArrayType<'ctx> {
  fn type_ref(&self) -> LLVMTypeRef {
    self.0
  }
}

impl<'ctx> FromLLVMType for ArrayType<'ctx> {
  fn from_llvm(ptr: LLVMTypeRef) -> Self {
    Self(ptr, PhantomData)
  }
}
