use llvm_sys::core::{LLVMGetElementType, LLVMGetNumOperands, LLVMGetOperand, LLVMIsTailCall};
use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

use crate::types::*;
use crate::values::*;
use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CallInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> HasType for CallInstruction<'ctx> {}

impl<'ctx> HasDebugLoc for CallInstruction<'ctx> {}

impl<'ctx> CallInstruction<'ctx> {
  pub fn callee_function(&self) -> Option<Function<'ctx>> {
    match self.callee() {
      Operand::Constant(Constant::Function(f)) => Some(f),
      _ => None,
    }
  }

  pub fn callee_function_type(&self) -> FunctionType<'ctx> {
    FunctionType::from_llvm(unsafe { LLVMGetElementType(self.callee().get_type().type_ref()) })
  }

  pub fn callee(&self) -> Operand<'ctx> {
    let operand_id = self.num_arguments();
    Operand::from_llvm(unsafe { LLVMGetOperand(self.0, operand_id as u32) })
  }

  pub fn num_arguments(&self) -> usize {
    let num_operands = unsafe { LLVMGetNumOperands(self.0) };
    num_operands as usize - 1
  }

  pub fn args(&self) -> Vec<Operand<'ctx>> {
    (0..self.num_arguments())
      .map(|i| Operand::from_llvm(unsafe { LLVMGetOperand(self.0, i as u32) }))
      .collect()
  }

  pub fn is_tail_call(&self) -> bool {
    unsafe { LLVMIsTailCall(self.0) == 1 }
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
