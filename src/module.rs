use llvm_sys::core::*;
use llvm_sys::prelude::{LLVMModuleRef, LLVMValueRef};
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// LLVM Module
pub struct Module<'ctx>(LLVMModuleRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for Module<'ctx> {}

unsafe impl<'ctx> Sync for Module<'ctx> {}

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

  /// Iterate all globals and global aliases inside the module
  ///
  /// ```
  /// for glob in mod.iter_globals() {
  ///   // Do things with glob
  /// }
  /// ```
  pub fn iter_globals(&self) -> ModuleGlobalIterator<'ctx> {
    let first_gl_ptr = unsafe { LLVMGetFirstGlobal(self.0) };
    ModuleGlobalIterator {
      module: self.0,
      curr: first_gl_ptr,
      is_global: true,
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
  module: LLVMModuleRef,
  curr: LLVMValueRef,
  is_global: bool,
  marker: PhantomData<&'ctx ()>,
}

impl<'ctx> Iterator for ModuleGlobalIterator<'ctx> {
  type Item = Global<'ctx>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.is_global {
      if self.curr.is_null() {
        self.is_global = false;
        let first_global_alias = unsafe { LLVMGetFirstGlobalAlias(self.module) };
        if first_global_alias.is_null() {
          None
        } else {
          self.curr = unsafe { LLVMGetNextGlobalAlias(first_global_alias) };
          Some(Global::from_llvm(first_global_alias))
        }
      } else {
        let result = Some(Global::from_llvm(self.curr));
        self.curr = unsafe { LLVMGetNextGlobal(self.curr) };
        result
      }
    } else {
      if self.curr.is_null() {
        None
      } else {
        let result = Some(Global::from_llvm(self.curr));
        self.curr = unsafe { LLVMGetNextGlobalAlias(self.curr) };
        result
      }
    }
  }
}
