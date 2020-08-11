use llvm_sys::core::{LLVMGetNumOperands, LLVMGetOperand};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

/// [Get Element Pointer (GEP) instruction](https://llvm.org/docs/LangRef.html#getelementptr-instruction)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GetElementPtrInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for GetElementPtrInstruction<'ctx> {}

unsafe impl<'ctx> Sync for GetElementPtrInstruction<'ctx> {}

impl<'ctx> GetType<'ctx> for GetElementPtrInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for GetElementPtrInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for GetElementPtrInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for GetElementPtrInstruction<'ctx> {}

impl<'ctx> GetElementPtrInstruction<'ctx> {
  /// Get the base location
  pub fn location(&self) -> Operand<'ctx> {
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  /// Get the number of indices used to get the element pointer
  pub fn num_indices(&self) -> usize {
    (unsafe { LLVMGetNumOperands(self.0) as usize }) - 1
  }

  /// Get the indices used to get the pointer, in vector form
  pub fn indices(&self) -> Vec<Operand<'ctx>> {
    (0..self.num_indices())
      .map(|i| Operand::from_llvm(unsafe { LLVMGetOperand(self.0, i as u32) }))
      .collect()
  }

  /// Get the pointer type of the result of this GEP instruction
  pub fn get_pointer_type(&self) -> PointerType<'ctx> {
    PointerType::from_llvm(self.get_type().type_ref())
  }
}

impl<'ctx> AsInstruction<'ctx> for GetElementPtrInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::GetElementPtr(*self)
  }
}

impl<'ctx> FromLLVMValue for GetElementPtrInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    GetElementPtrInstruction(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for GetElementPtrInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
