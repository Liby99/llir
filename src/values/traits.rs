use llvm_sys::core::LLVMTypeOf;

use crate::types::*;
use crate::*;

pub trait HasType {}

pub trait GetType<'ctx> {
  fn get_type(&self) -> Type<'ctx>;
}

impl<'ctx, V> GetType<'ctx> for V
where
  V: ValueRef + HasType,
{
  fn get_type(&self) -> Type<'ctx> {
    Type::from_llvm(unsafe { LLVMTypeOf(self.value_ref()) })
  }
}
