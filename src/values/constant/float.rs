use llvm_sys::core::LLVMConstRealGetDouble;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

/// [Float constant](https://llvm.org/docs/LangRef.html#simple-constants)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FloatConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for FloatConstant<'ctx> {}

unsafe impl<'ctx> Sync for FloatConstant<'ctx> {}

impl<'ctx> GetType<'ctx> for FloatConstant<'ctx> {}

impl<'ctx> FloatConstant<'ctx> {
  /// Get the floating point value in double form (f64)
  pub fn double_value(&self) -> f64 {
    unsafe {
      let mut b = 0;
      let b_ptr: *mut std::os::raw::c_int = &mut b;
      LLVMConstRealGetDouble(self.0, b_ptr)
    }
  }

  /// Get directly the float type
  pub fn get_float_type(&self) -> FloatType<'ctx> {
    FloatType::from_llvm(self.get_type().type_ref())
  }
}

impl<'ctx> ValueRef for FloatConstant<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}

impl<'ctx> FromLLVMValue for FloatConstant<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> AsConstant<'ctx> for FloatConstant<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Float(self.clone())
  }
}

impl_as_operand_for_constant!(FloatConstant);
