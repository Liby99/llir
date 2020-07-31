use llvm_sys::core::LLVMGetConstOpcode;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::value::BinaryOpcode;
use crate::{FromLLVMValue, ValueRef};

#[derive(Copy, Clone)]
pub struct BinaryConstExpr<'ctx>(BinaryOpcode, LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> FromLLVMValue for BinaryConstExpr<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    let opcode = BinaryOpcode::from_llvm(unsafe { LLVMGetConstOpcode(ptr) }).unwrap();
    Self(opcode, ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for BinaryConstExpr<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.1
  }
}