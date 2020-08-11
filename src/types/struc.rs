use llvm_sys::core::{LLVMCountStructElementTypes, LLVMGetStructElementTypes, LLVMIsLiteralStruct};
use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::utils::string_of_type;
use crate::{FromLLVMType, TypeRef};

/// Struct type, could either be named or a literal
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StructType<'ctx> {
  NamedStruct(NamedStructType<'ctx>),
  LiteralStruct(LiteralStructType<'ctx>),
}

pub trait StructTypeTrait<'ctx>: TypeRef {
  /// Get the number of element types in the Struct
  fn num_element_types(&self) -> usize {
    unsafe { LLVMCountStructElementTypes(self.type_ref()) as usize }
  }

  /// Get the element types in vector
  fn element_types(&self) -> Vec<Type<'ctx>> {
    let num_elems = self.num_element_types();
    let mut type_refs = Vec::with_capacity(num_elems);
    unsafe {
      LLVMGetStructElementTypes(self.type_ref(), type_refs.as_mut_ptr());
      type_refs.set_len(num_elems);
    };
    type_refs.into_iter().map(|t| Type::from_llvm(t)).collect()
  }

  /// Get the element type at a given index
  fn element_type(&self, index: usize) -> Option<Type<'ctx>> {
    let types = self.element_types();
    if index < types.len() {
      Some(types[index])
    } else {
      None
    }
  }
}

impl<'ctx> StructType<'ctx> {
  /// Check if the struct type is named
  pub fn is_named(&self) -> bool {
    match self {
      Self::NamedStruct(_) => true,
      Self::LiteralStruct(_) => false,
    }
  }

  /// Get the struct name
  pub fn name(&self) -> Option<String> {
    match self {
      Self::NamedStruct(ns) => Some(ns.name()),
      _ => None,
    }
  }
}

impl<'ctx> AsType<'ctx> for StructType<'ctx> {
  fn as_type(&self) -> Type<'ctx> {
    Type::Struct(self.clone())
  }
}

impl<'ctx> TypeRef for StructType<'ctx> {
  fn type_ref(&self) -> LLVMTypeRef {
    match self {
      Self::NamedStruct(ns) => ns.type_ref(),
      Self::LiteralStruct(ls) => ls.type_ref(),
    }
  }
}

impl<'ctx> FromLLVMType for StructType<'ctx> {
  fn from_llvm(ptr: LLVMTypeRef) -> Self {
    if unsafe { LLVMIsLiteralStruct(ptr) } != 0 {
      Self::LiteralStruct(LiteralStructType::from_llvm(ptr))
    } else {
      Self::NamedStruct(NamedStructType::from_llvm(ptr))
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LiteralStructType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

impl<'ctx> StructTypeTrait<'ctx> for LiteralStructType<'ctx> {}

impl<'ctx> AsType<'ctx> for LiteralStructType<'ctx> {
  fn as_type(&self) -> Type<'ctx> {
    Type::Struct(StructType::LiteralStruct(self.clone()))
  }
}

impl<'ctx> TypeRef for LiteralStructType<'ctx> {
  fn type_ref(&self) -> LLVMTypeRef {
    self.0
  }
}

impl<'ctx> FromLLVMType for LiteralStructType<'ctx> {
  fn from_llvm(ptr: LLVMTypeRef) -> Self {
    Self(ptr, PhantomData)
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NamedStructType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

impl<'ctx> StructTypeTrait<'ctx> for NamedStructType<'ctx> {}

impl<'ctx> NamedStructType<'ctx> {
  /// Get the name of the named struct
  pub fn name(&self) -> String {
    string_of_type(self.0)
  }
}

impl<'ctx> AsType<'ctx> for NamedStructType<'ctx> {
  fn as_type(&self) -> Type<'ctx> {
    Type::Struct(StructType::NamedStruct(self.clone()))
  }
}

impl<'ctx> TypeRef for NamedStructType<'ctx> {
  fn type_ref(&self) -> LLVMTypeRef {
    self.0
  }
}

impl<'ctx> FromLLVMType for NamedStructType<'ctx> {
  fn from_llvm(ptr: LLVMTypeRef) -> Self {
    Self(ptr, PhantomData)
  }
}
