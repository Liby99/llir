use llvm_sys::core::*;
use llvm_sys::prelude::{LLVMModuleRef, LLVMValueRef};
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// LLVM Module
pub struct Module<'ctx>(LLVMModuleRef, PhantomData<&'ctx ()>);

impl_send_sync!(Module);

impl<'ctx> Module<'ctx> {
  pub(crate) fn new(ptr: LLVMModuleRef) -> Self {
    Self(ptr, PhantomData)
  }

  /// Iterate all functions inside the module
  ///
  /// ```
  /// # use llir::*;
  /// # use std::path::*;
  /// # let ctx = Context::create();
  /// # let module = ctx.load_module(PathBuf::from("tests/c_files/basic/example_1.bc"));
  /// for func in module.iter_functions() {
  ///   // Do things with func
  /// }
  /// ```
  pub fn iter_functions(&self) -> ValueIterator<'ctx, Function<'ctx>> {
    ValueIterator {
      curr_value: unsafe { LLVMGetFirstFunction(self.0) },
      next_fn: next_function,
      marker: PhantomData
    }
  }

  /// Get the function by name
  pub fn get_function(&self, name: &str) -> Option<Function<'ctx>> {
    let zero_appended = format!("{}\0", name);
    let fn_ptr = unsafe { LLVMGetNamedFunction(self.0, zero_appended.as_ptr() as *const i8) };
    if fn_ptr.is_null() { None } else { Some(Function::from_llvm(fn_ptr)) }
  }

  /// Iterate all globals variables
  ///
  /// ```
  /// for glob in module.iter_global_variables() {
  ///   // Do things with glob
  /// }
  /// ```
  pub fn iter_global_variables(&self) -> ValueIterator<'ctx, GlobalVariable<'ctx>> {
    ValueIterator {
      curr_value: unsafe { LLVMGetFirstGlobal(self.0) },
      next_fn: next_global,
      marker: PhantomData
    }
  }

  /// Get global variable by name
  pub fn get_global_variable(&self, name: &str) -> Option<GlobalVariable<'ctx>> {
    let zero_appended = format!("{}\0", name);
    let gv_ptr = unsafe { LLVMGetNamedGlobal(self.0, zero_appended.as_ptr() as *const i8) };
    if gv_ptr.is_null() { None } else { Some(GlobalVariable::from_llvm(gv_ptr)) }
  }

  /// Iterate all global aliases
  ///
  /// ```
  /// for glob_alias in module.iter_global_aliases() {
  ///   // Do things with global alias
  /// }
  /// ```
  pub fn iter_global_aliases(&self) -> ValueIterator<'ctx, GlobalAlias<'ctx>> {
    ValueIterator {
      curr_value: unsafe { LLVMGetFirstGlobalAlias(self.0) },
      next_fn: next_global_alias,
      marker: PhantomData,
    }
  }

  /// Get global alias by name
  pub fn get_global_alias(&self, name: &str) -> Option<GlobalAlias<'ctx>> {
    let ga_ptr = unsafe { LLVMGetNamedGlobalAlias(self.0, name.as_ptr() as *const i8, name.len()) };
    if ga_ptr.is_null() { None } else { Some(GlobalAlias::from_llvm(ga_ptr)) }
  }

  /// Iterate all globals, including global variables and global aliases
  ///
  /// ``` rust
  /// for glob in module.iter_globals() {
  ///   match glob {
  ///     Global::Variable(v) => { /* ... */ },
  ///     Global::Alias(a) => { /* ... */ },
  ///   }
  /// }
  /// ```
  pub fn iter_globals(&self) -> GlobalIterator<'ctx> {
    GlobalIterator {
      var_iter: self.iter_global_variables(),
      alias_iter: self.iter_global_aliases(),
    }
  }
}

fn next_function(ptr: LLVMValueRef) -> LLVMValueRef {
  unsafe { LLVMGetNextFunction(ptr) }
}

fn next_global(ptr: LLVMValueRef) -> LLVMValueRef {
  unsafe { LLVMGetNextGlobal(ptr) }
}

fn next_global_alias(ptr: LLVMValueRef) -> LLVMValueRef {
  unsafe { LLVMGetNextGlobalAlias(ptr) }
}

#[doc(hidden)]
pub struct ValueIterator<'ctx, T> where T : FromLLVMValue {
  curr_value: LLVMValueRef,
  next_fn: fn(LLVMValueRef) -> LLVMValueRef,
  marker: PhantomData<&'ctx T>,
}

impl<'ctx, T> Iterator for ValueIterator<'ctx, T> where T : FromLLVMValue {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.curr_value.is_null() {
      None
    } else {
      let result = Some(T::from_llvm(self.curr_value));
      self.curr_value = (self.next_fn)(self.curr_value);
      result
    }
  }
}

#[doc(hidden)]
pub struct GlobalIterator<'ctx> {
  var_iter: ValueIterator<'ctx, GlobalVariable<'ctx>>,
  alias_iter: ValueIterator<'ctx, GlobalAlias<'ctx>>,
}

impl<'ctx> Iterator for GlobalIterator<'ctx> {
  type Item = Global<'ctx>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.var_iter.next() {
      Some(gv) => Some(gv.as_global()),
      None => match self.alias_iter.next() {
        Some(ga) => Some(ga.as_global()),
        None => None
      }
    }
  }
}