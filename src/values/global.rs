use llvm_sys::core::*;
use llvm_sys::LLVMValueKind;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::utils::string_of_value;
use crate::values::*;
use crate::*;

/// Global value
///
/// The value kind could either be global variable or global alias
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Global<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> GetType<'ctx> for Global<'ctx> {}

impl<'ctx> Global<'ctx> {
  /// Get the name of this global
  pub fn name(&self) -> String {
    string_of_value(self.0)
  }

  /// Check if the global is a global alias
  pub fn is_alias(&self) -> bool {
    use LLVMValueKind::*;
    match unsafe { LLVMGetValueKind(self.0) } {
      LLVMGlobalAliasValueKind => true,
      _ => false
    }
  }
}

impl<'ctx> FromLLVMValue for Global<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for Global<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsConstant<'ctx> for Global<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Global(self.clone())
  }
}
