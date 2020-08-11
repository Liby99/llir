use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

/// [Alloca instruction](https://llvm.org/docs/LangRef.html#alloca-instruction)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AllocaInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for AllocaInstruction<'ctx> {}

unsafe impl<'ctx> Sync for AllocaInstruction<'ctx> {}

impl<'ctx> GetType<'ctx> for AllocaInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for AllocaInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for AllocaInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for AllocaInstruction<'ctx> {}

impl<'ctx> AsInstruction<'ctx> for AllocaInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Alloca(*self)
  }
}

impl<'ctx> AllocaInstruction<'ctx> {
  /// Get the pointer type of alloca
  pub fn get_pointer_type(&self) -> PointerType<'ctx> {
    PointerType::from_llvm(self.get_type().type_ref())
  }

  /// Get the element type which this allocated pointer points to
  pub fn get_element_type(&self) -> Type<'ctx> {
    self.get_pointer_type().element_type()
  }
}

impl<'ctx> FromLLVMValue for AllocaInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    AllocaInstruction(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for AllocaInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
