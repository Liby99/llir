use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMValueKind;
use std::marker::PhantomData;

use crate::utils::string_of_value;
use crate::values::*;
use crate::*;

/// Global container enum
///
/// A global could be either a normal variable or an alias
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Global<'ctx> {
  /// Global variable
  Variable(GlobalVariable<'ctx>),
  /// Global alias
  Alias(GlobalAlias<'ctx>),
}

pub trait GlobalValueTrait<'ctx>: ValueRef {
  /// Get the name of this global
  fn name(&self) -> String {
    string_of_value(self.value_ref())
  }
}

impl<'ctx> GetType<'ctx> for Global<'ctx> {}

impl<'ctx> GlobalValueTrait<'ctx> for Global<'ctx> {}

impl<'ctx> Global<'ctx> {
  /// Check if a global variable is alias
  pub fn is_alias(&self) -> bool {
    match self {
      Self::Variable(_) => false,
      _ => true,
    }
  }

  /// Get the aliasee
  pub fn aliasee(&self) -> Option<Constant<'ctx>> {
    match self {
      Self::Alias(a) => Some(a.aliasee()),
      _ => None,
    }
  }
}

impl<'ctx> FromLLVMValue for Global<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    use LLVMValueKind::*;
    match unsafe { LLVMGetValueKind(ptr) } {
      LLVMGlobalVariableValueKind => Self::Variable(GlobalVariable::from_llvm(ptr)),
      LLVMGlobalAliasValueKind => Self::Alias(GlobalAlias::from_llvm(ptr)),
      x => panic!("Not global variable kind {:?}", x),
    }
  }
}

impl<'ctx> ValueRef for Global<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    match self {
      Self::Variable(v) => v.value_ref(),
      Self::Alias(a) => a.value_ref(),
    }
  }
}

impl<'ctx> AsConstant<'ctx> for Global<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Global(self.clone())
  }
}

/// Global value
///
/// The value kind could either be global variable or global alias
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GlobalVariable<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for GlobalVariable<'ctx> {}

unsafe impl<'ctx> Sync for GlobalVariable<'ctx> {}

impl<'ctx> GetType<'ctx> for GlobalVariable<'ctx> {}

impl<'ctx> GlobalValueTrait<'ctx> for GlobalVariable<'ctx> {}

impl<'ctx> GlobalVariable<'ctx> {
  /// Get the name of this global
  pub fn name(&self) -> String {
    string_of_value(self.0)
  }
}

impl<'ctx> FromLLVMValue for GlobalVariable<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for GlobalVariable<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsConstant<'ctx> for GlobalVariable<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Global(Global::Variable(self.clone()))
  }
}

/// Global alias
///
/// The value kind could either be global variable or global alias
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GlobalAlias<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for GlobalAlias<'ctx> {}

unsafe impl<'ctx> Sync for GlobalAlias<'ctx> {}

impl<'ctx> GetType<'ctx> for GlobalAlias<'ctx> {}

impl<'ctx> GlobalValueTrait<'ctx> for GlobalAlias<'ctx> {}

impl<'ctx> GlobalAlias<'ctx> {
  /// Get the aliasee of the global alias
  pub fn aliasee(&self) -> Constant<'ctx> {
    Constant::from_llvm(unsafe { LLVMAliasGetAliasee(self.0) })
  }
}

impl<'ctx> FromLLVMValue for GlobalAlias<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for GlobalAlias<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> AsConstant<'ctx> for GlobalAlias<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Global(Global::Alias(self.clone()))
  }
}
