use llvm_sys::core::*;
use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::{FromLLVMType, TypeRef};

/// [Function Type](https://llvm.org/docs/LangRef.html#function-type)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FunctionType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

impl_send_sync!(FunctionType);

impl<'ctx> FunctionType<'ctx> {
  /// Get the number of argument types
  pub fn num_argument_types(&self) -> usize {
    unsafe { LLVMCountParamTypes(self.0) as usize }
  }

  /// Get argument types in an array
  pub fn argument_types(&self) -> Vec<Type<'ctx>> {
    let num_arg_types = self.num_argument_types();
    let mut type_refs = Vec::with_capacity(num_arg_types);
    unsafe {
      LLVMGetParamTypes(self.0, type_refs.as_mut_ptr());
      type_refs.set_len(num_arg_types);
    };
    type_refs.into_iter().map(|t| Type::from_llvm(t)).collect()
  }

  /// Get the argument type at a given index
  pub fn argument_type(&self, index: usize) -> Option<Type<'ctx>> {
    let types = self.argument_types();
    if index < types.len() {
      Some(types[index])
    } else {
      None
    }
  }

  /// Get the return type
  pub fn return_type(&self) -> Type<'ctx> {
    Type::from_llvm(unsafe { LLVMGetReturnType(self.0) })
  }

  /// Check if the return type is not a void type
  pub fn has_return_type(&self) -> bool {
    match self.return_type() {
      Type::Void(_) => false,
      _ => true,
    }
  }

  /// Check if the function type is variable argument
  pub fn is_var_arg(&self) -> bool {
    unsafe { LLVMIsFunctionVarArg(self.0) != 0 }
  }
}

impl<'ctx> AsType<'ctx> for FunctionType<'ctx> {
  fn as_type(&self) -> Type<'ctx> {
    Type::Function(self.clone())
  }
}

impl_positional_type_ref!(FunctionType, 0);

impl_positional_from_llvm_type!(FunctionType);
