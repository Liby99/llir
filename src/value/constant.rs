use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use super::FromLLVM;

#[derive(Copy, Clone)]
pub enum Constant<'ctx> {
  Int(IntConstant<'ctx>),
  Float(FloatConstant<'ctx>),
  Null(NullConstant<'ctx>),
}

impl<'ctx> FromLLVM for Constant<'ctx> {
  fn from_llvm(_ptr: LLVMValueRef) -> Self {
    panic!("Not implemented");
  }
}

#[derive(Copy, Clone)]
pub struct IntConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

#[derive(Copy, Clone)]
pub struct FloatConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

#[derive(Copy, Clone)]
pub struct NullConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);
