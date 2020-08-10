use llvm_sys::core::*;
use llvm_sys::prelude::{LLVMModuleRef, LLVMValueRef};
use std::marker::PhantomData;

use crate::*;
use crate::values::*;

/// LLVM Module
pub struct Module<'ctx>(LLVMModuleRef, PhantomData<&'ctx ()>);

impl<'ctx> Module<'ctx> {
  pub(crate) fn new(ptr: LLVMModuleRef) -> Self {
    Self(ptr, PhantomData)
  }

  /// Iterate all functions inside the module
  ///
  /// ```
  /// for func in mod.iter_functions() {
  ///   // Do things with func
  /// }
  /// ```
  pub fn iter_functions(&self) -> ModuleFunctionIterator<'ctx> {
    let first_fn_ptr = unsafe { LLVMGetFirstFunction(self.0) };
    ModuleFunctionIterator {
      curr_function: first_fn_ptr,
      marker: PhantomData,
    }
  }

  /// Iterate all globals inside the module
  ///
  /// ```
  /// for glob in mod.iter_globals() {
  ///   // Do things with glob
  /// }
  /// ```
  pub fn iter_globals(&self) -> ModuleGlobalIterator<'ctx> {
    let first_gl_ptr = unsafe { LLVMGetFirstGlobal(self.0) };
    ModuleGlobalIterator {
      curr_global: first_gl_ptr,
      marker: PhantomData,
    }
  }
}

#[doc(hidden)]
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

#[doc(hidden)]
pub struct ModuleGlobalIterator<'ctx> {
  curr_global: LLVMValueRef,
  marker: PhantomData<&'ctx ()>,
}

impl<'ctx> Iterator for ModuleGlobalIterator<'ctx> {
  type Item = Global<'ctx>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.curr_global.is_null() {
      None
    } else {
      let result = Some(Global::from_llvm(self.curr_global));
      self.curr_global = unsafe { LLVMGetNextGlobal(self.curr_global) };
      result
    }
  }
}