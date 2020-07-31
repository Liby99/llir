use llvm_sys::core::LLVMGetConstOpcode;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::value::UnaryOpcode;
use crate::{FromLLVMValue, ValueRef};

#[derive(Copy, Clone)]
pub struct UnaryConstExpr<'ctx>(UnaryOpcode, LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> FromLLVMValue for UnaryConstExpr<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    let opcode = UnaryOpcode::from_llvm(unsafe { LLVMGetConstOpcode(ptr) }).unwrap();
    Self(opcode, ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for UnaryConstExpr<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.1
  }
}