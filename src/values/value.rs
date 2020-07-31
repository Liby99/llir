use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

pub struct Value<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> Value<'ctx> {
  pub(crate) fn new(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}