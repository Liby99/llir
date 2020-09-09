use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMValueKind;
use std::marker::PhantomData;

use crate::utils::string_of_value;
use crate::values::*;
use crate::*;

/// Global container enum
///
/// A global could be either a [normal variable](https://llvm.org/docs/LangRef.html#global-variables)
/// or an [alias](https://llvm.org/docs/LangRef.html#aliases)
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

  /// Global value can be turned into a Global enum
  fn as_global(&self) -> Global<'ctx>;
}

/// A global variable has a type
impl<'ctx> GetType<'ctx> for Global<'ctx> {}

/// Global variable implements the trait for global value
impl<'ctx> GlobalValueTrait<'ctx> for Global<'ctx> {
  fn as_global(&self) -> Self { self.clone() }
}

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

/// We can turn a Global into a Global Alias Constant
impl<'ctx> AsConstant<'ctx> for Global<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Global(self.clone())
  }
}

impl_as_operand_for_constant!(Global);

/// [Global variable](https://llvm.org/docs/LangRef.html#global-variables)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct GlobalVariable<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> std::fmt::Debug for GlobalVariable<'ctx> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_tuple("GlobalVariable").field(&self.name()).finish()
  }
}

impl_send_sync!(GlobalVariable);

/// A global value has a type
impl<'ctx> GetType<'ctx> for GlobalVariable<'ctx> {}

/// Global value implements the trait for global value
impl<'ctx> GlobalValueTrait<'ctx> for GlobalVariable<'ctx> {
  fn as_global(&self) -> Global<'ctx> {
    Global::Variable(self.clone())
  }
}

impl_positional_value_ref!(GlobalVariable, 0);

impl_positional_from_llvm_value!(GlobalVariable);

/// We can turn Global Variable into a Global Alias Constant
impl<'ctx> AsConstant<'ctx> for GlobalVariable<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Global(Global::Variable(self.clone()))
  }
}

impl_as_operand_for_constant!(GlobalVariable);

/// [Global alias](https://llvm.org/docs/LangRef.html#aliases)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct GlobalAlias<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> std::fmt::Debug for GlobalAlias<'ctx> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_tuple("GlobalAlias").field(&self.name()).finish()
  }
}

impl_send_sync!(GlobalAlias);

/// A global alias has a type
impl<'ctx> GetType<'ctx> for GlobalAlias<'ctx> {}

/// Global alias implements the trait for global value
impl<'ctx> GlobalValueTrait<'ctx> for GlobalAlias<'ctx> {
  fn as_global(&self) -> Global<'ctx> {
    Global::Alias(self.clone())
  }
}

impl<'ctx> GlobalAlias<'ctx> {
  /// Get the aliasee of the global alias
  pub fn aliasee(&self) -> Constant<'ctx> {
    Constant::from_llvm(unsafe { LLVMAliasGetAliasee(self.0) })
  }
}

impl_positional_value_ref!(GlobalAlias, 0);

impl_positional_from_llvm_value!(GlobalAlias);

/// We can turn Global Alias into a Global Alias Constant
impl<'ctx> AsConstant<'ctx> for GlobalAlias<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Global(Global::Alias(self.clone()))
  }
}

impl_as_operand_for_constant!(GlobalAlias);
