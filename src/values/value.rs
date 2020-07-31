use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Value<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> Value<'ctx> {
  pub(crate) fn new(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}