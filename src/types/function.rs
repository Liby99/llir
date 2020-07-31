use llvm_sys::core::{LLVMCountParamTypes, LLVMGetParamTypes, LLVMGetReturnType, LLVMIsFunctionVarArg};
use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use super::Type;
use crate::{FromLLVMType, TypeRef};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FunctionType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

impl<'ctx> FunctionType<'ctx> {
  pub fn num_param_types(&self) -> usize {
    unsafe { LLVMCountParamTypes(self.0) as usize }
  }

  pub fn param_types(&self) -> Vec<Type<'ctx>> {
    let num_param_types = self.num_param_types();
    let mut type_refs = Vec::with_capacity(num_param_types);
    unsafe {
      LLVMGetParamTypes(self.0, type_refs.as_mut_ptr());
      type_refs.set_len(num_param_types);
    };
    type_refs.into_iter().map(|t| Type::from_llvm(t)).collect()
  }

  pub fn return_type(&self) -> Type<'ctx> {
    Type::from_llvm(unsafe { LLVMGetReturnType(self.0) })
  }

  pub fn is_var_arg(&self) -> bool {
    unsafe { LLVMIsFunctionVarArg(self.0) != 0 }
  }
}

impl<'ctx> TypeRef for FunctionType<'ctx> {
  fn type_ref(&self) -> LLVMTypeRef {
    self.0
  }
}

impl<'ctx> FromLLVMType for FunctionType<'ctx> {
  fn from_llvm(ptr: LLVMTypeRef) -> Self {
    Self(ptr, PhantomData)
  }
}
