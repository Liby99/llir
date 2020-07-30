use std::marker::PhantomData;
use llvm_sys::prelude::{LLVMValueRef};

#[derive(Copy, Clone)]
pub enum Constant<'ctx> {
  Int(IntConstant<'ctx>),
  Float(FloatConstant<'ctx>),
  Null(NullConstant<'ctx>),
}

#[derive(Copy, Clone)]
pub struct IntConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

#[derive(Copy, Clone)]
pub struct FloatConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

#[derive(Copy, Clone)]
pub struct NullConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);