use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

/// [CallBr instruction](https://llvm.org/docs/LangRef.html#callbr-instruction)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CallBrInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for CallBrInstruction<'ctx> {}

unsafe impl<'ctx> Sync for CallBrInstruction<'ctx> {}

impl<'ctx> GetType<'ctx> for CallBrInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for CallBrInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for CallBrInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for CallBrInstruction<'ctx> {}

impl<'ctx> CallBrInstruction<'ctx> {}

impl<'ctx> AsInstruction<'ctx> for CallBrInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::CallBr(*self)
  }
}

impl<'ctx> FromLLVMValue for CallBrInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for CallBrInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
