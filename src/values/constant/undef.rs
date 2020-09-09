use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// [Undefined value](https://llvm.org/docs/LangRef.html#undefined-values)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Undef<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_send_sync!(Undef);

impl<'ctx> GetType<'ctx> for Undef<'ctx> {}

impl_positional_value_ref!(Undef, 0);

impl_positional_from_llvm_value!(Undef);

impl<'ctx> AsConstant<'ctx> for Undef<'ctx> {
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::Undef(self.clone())
  }
}

impl_as_operand_for_constant!(Undef);
