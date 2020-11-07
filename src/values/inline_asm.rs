use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

/// Inline Assembly value
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct InlineAsm<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl_send_sync!(InlineAsm);

impl<'ctx> InlineAsm<'ctx> {
  /// Get the type of this InlineAsm in function type form
  pub fn function_type(&self) -> FunctionType<'ctx> {
    FunctionType::from_llvm(unsafe { LLVMGetElementType(LLVMTypeOf(self.0)) })
  }

  /// Get the whole string for inline asm
  ///
  /// General Format:
  /// {type} asm [[flags]] "{assembly_body}" "{constraints}"
  ///
  /// E.g.
  /// ``` llvm
  /// void (i64*, i8, i64*)* asm sideeffect ".pushsection .smp_locks,\22a\22\0A.balign 4\0A.long 671f - .\0A.popsection\0A671:\0A\09lock; orb $1,$0", "=*m,iq,*m,~{memory},~{dirflag},~{fpsr},~{flags}"
  /// ```
  pub fn to_string(&self) -> String {
    unsafe { utils::raw_to_string(LLVMPrintValueToString(self.value_ref())) }
  }
}

impl<'ctx> AsOperand<'ctx> for InlineAsm<'ctx> {
  fn as_operand(&self) -> Operand<'ctx> {
    Operand::InlineAsm(self.clone())
  }
}

impl_positional_value_ref!(InlineAsm, 0);

impl_positional_from_llvm_value!(InlineAsm);
