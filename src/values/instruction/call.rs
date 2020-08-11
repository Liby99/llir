use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

/// [Call instruction](https://llvm.org/docs/LangRef.html#call-instruction)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CallInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for CallInstruction<'ctx> {}

unsafe impl<'ctx> Sync for CallInstruction<'ctx> {}

impl<'ctx> GetType<'ctx> for CallInstruction<'ctx> {}

impl<'ctx> GetDebugMetadata<'ctx> for CallInstruction<'ctx> {}

impl<'ctx> InstructionDebugLoc for CallInstruction<'ctx> {}

impl<'ctx> InstructionTrait<'ctx> for CallInstruction<'ctx> {}

impl<'ctx> CallInstruction<'ctx> {
  /// Get the callee function if the callee is an LLVM function
  pub fn callee_function(&self) -> Option<Function<'ctx>> {
    match self.callee() {
      Operand::Constant(Constant::Function(f)) => Some(f),
      _ => None,
    }
  }

  /// Get the callee as inline assembly value if the callee is an InlineAsm
  pub fn callee_inline_asm(&self) -> Option<InlineAsm<'ctx>> {
    match self.callee() {
      Operand::InlineAsm(ia) => Some(ia),
      _ => None,
    }
  }

  /// Get the callee function type
  pub fn callee_function_type(&self) -> FunctionType<'ctx> {
    FunctionType::from_llvm(unsafe { LLVMGetElementType(self.callee().get_type().type_ref()) })
  }

  /// Get the callee value in operand
  pub fn callee(&self) -> Operand<'ctx> {
    let operand_id = self.num_arguments();
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, operand_id as u32) })
  }

  /// Get the number of arguments passed to the callee function
  pub fn num_arguments(&self) -> usize {
    let num_operands = unsafe { LLVMGetNumOperands(self.0) };
    num_operands as usize - 1
  }

  /// Get the arguments being passed to the callee function
  pub fn arguments(&self) -> Vec<Operand<'ctx>> {
    (0..self.num_arguments())
      .map(|i| Operand::from_llvm(unsafe { LLVMGetOperand(self.0, i as u32) }))
      .collect()
  }

  /// Get the argument at a given index
  pub fn argument(&self, index: usize) -> Option<Operand<'ctx>> {
    if index < self.num_arguments() {
      Some(Operand::from_llvm(unsafe { LLVMGetOperand(self.0, index as u32) }))
    } else {
      None
    }
  }

  /// Check if this function call is a tail call
  pub fn is_tail_call(&self) -> bool {
    unsafe { LLVMIsTailCall(self.0) == 1 }
  }

  /// Check if this call is to an inline assembly
  pub fn is_inline_asm_call(&self) -> bool {
    match self.callee() {
      Operand::InlineAsm(_) => true,
      _ => false
    }
  }

  /// Check if this call is to llvm intrinsic function
  pub fn is_intrinsic_call(&self) -> bool {
    unsafe { !LLVMIsAIntrinsicInst(self.0).is_null() }
  }
}

impl<'ctx> AsInstruction<'ctx> for CallInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx> {
    Instruction::Call(*self)
  }
}

impl<'ctx> FromLLVMValue for CallInstruction<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    CallInstruction(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for CallInstruction<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
