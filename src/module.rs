use llvm_sys::core::{LLVMGetFirstFunction, LLVMGetNextFunction};
use llvm_sys::prelude::{LLVMModuleRef, LLVMValueRef};
use std::marker::PhantomData;

use super::{value::Function, FromLLVMValue};

pub struct Module<'ctx>(LLVMModuleRef, PhantomData<&'ctx ()>);

impl<'ctx> Module<'ctx> {
  pub(crate) fn new(ptr: LLVMModuleRef) -> Self {
    Self(ptr, PhantomData)
  }

  pub fn iter_functions(&self) -> ModuleFunctionIterator<'ctx> {
    let first_fn_ptr = unsafe { LLVMGetFirstFunction(self.0) };
    ModuleFunctionIterator {
      curr_function: first_fn_ptr,
      marker: PhantomData,
    }
  }
}

pub struct ModuleFunctionIterator<'ctx> {
  curr_function: LLVMValueRef,
  marker: PhantomData<&'ctx ()>,
}

impl<'ctx> Iterator for ModuleFunctionIterator<'ctx> {
  type Item = Function<'ctx>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.curr_function.is_null() {
      None
    } else {
      let result = Some(Function::from_llvm(self.curr_function));
      self.curr_function = unsafe { LLVMGetNextFunction(self.curr_function) };
      result
    }
  }
}
