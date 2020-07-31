use llvm_sys::core::{LLVMIsLiteralStruct, LLVMCountStructElementTypes, LLVMGetStructElementTypes};
use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use super::Type;
use crate::utils::string_of_type;
use crate::{TypeRef, FromLLVMType};

#[derive(Copy, Clone)]
pub enum StructType<'ctx> {
  NamedStruct(NamedStructType<'ctx>),
  LiteralStruct(LiteralStructType<'ctx>),
}

impl<'ctx> StructType<'ctx> {
  pub fn is_named(&self) -> bool {
    match self {
      Self::NamedStruct(_) => true,
      Self::LiteralStruct(_) => false,
    }
  }

  pub fn num_element_types(&self) -> usize {
    match self {
      Self::NamedStruct(ns) => ns.num_element_types(),
      Self::LiteralStruct(ls) => ls.num_element_types(),
    }
  }

  pub fn element_types(&self) -> Vec<Type<'ctx>> {
    match self {
      Self::NamedStruct(ns) => ns.element_types(),
      Self::LiteralStruct(ls) => ls.element_types(),
    }
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

#[derive(Copy, Clone)]
pub struct LiteralStructType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

impl<'ctx> LiteralStructType<'ctx> {
  pub fn num_element_types(&self) -> usize {
    struct_num_element_types(self.0)
  }

  pub fn element_types(&self) -> Vec<Type<'ctx>> {
    struct_element_types(self.0)
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

#[derive(Copy, Clone)]
pub struct NamedStructType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

impl<'ctx> NamedStructType<'ctx> {
  pub fn name(&self) -> String {
    string_of_type(self.0)
  }

  pub fn num_element_types(&self) -> usize {
    struct_num_element_types(self.0)
  }

  pub fn element_types(&self) -> Vec<Type<'ctx>> {
    struct_element_types(self.0)
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

fn struct_num_element_types(ty: LLVMTypeRef) -> usize {
  unsafe { LLVMCountStructElementTypes(ty) as usize }
}

fn struct_element_types<'ctx>(ty: LLVMTypeRef) -> Vec<Type<'ctx>> {
  let num_elems = struct_num_element_types(ty);
  let mut type_refs = Vec::with_capacity(num_elems);
  unsafe {
    LLVMGetStructElementTypes(ty, type_refs.as_mut_ptr());
    type_refs.set_len(num_elems);
  };
  type_refs.into_iter().map(|t| Type::from_llvm(t)).collect()
}