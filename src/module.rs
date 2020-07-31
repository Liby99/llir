use llvm_sys::prelude::LLVMModuleRef;
use std::marker::PhantomData;

pub struct Module<'ctx>(LLVMModuleRef, PhantomData<&'ctx ()>);

impl<'ctx> Module<'ctx> {
  pub(crate) fn new(ptr: LLVMModuleRef) -> Self {
    Self(ptr, PhantomData)
  }
}
