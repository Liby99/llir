use llvm_sys::core::{LLVMGetNumOperands, LLVMGetOperand, LLVMGetValueKind};
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMValueKind;
use std::marker::PhantomData;

use crate::values::*;
use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GetElementPtrConstExpr<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> GetElementPtrConstExpr<'ctx> {
  pub fn location(&self) -> Constant<'ctx> {
    Constant::from_llvm(unsafe { LLVMGetOperand(self.0, 0) })
  }

  pub fn num_indices(&self) -> usize {
    (unsafe { LLVMGetNumOperands(self.0) as usize }) - 1
  }

  pub fn indices(&self) -> Vec<Constant<'ctx>> {
    (0..self.num_indices() as u32)
      .map(|i| Constant::from_llvm(unsafe { LLVMGetOperand(self.0, i) }))
      .collect()
  }

  pub fn int_indices(&self) -> Vec<IntConstant<'ctx>> {
    (0..self.num_indices() as u32)
      .map(|i| {
        let operand = unsafe { LLVMGetOperand(self.0, i) };
        assert_eq!(
          unsafe { LLVMGetValueKind(operand) },
          LLVMValueKind::LLVMConstantIntValueKind
        );
        IntConstant::from_llvm(operand)
      })
      .collect()
  }
}

impl<'ctx> FromLLVMValue for GetElementPtrConstExpr<'ctx> {
  fn from_llvm(ptr: LLVMValueRef) -> Self {
    Self(ptr, PhantomData)
  }
}

impl<'ctx> ValueRef for GetElementPtrConstExpr<'ctx> {
  fn value_ref(&self) -> LLVMValueRef {
    self.0
  }
}
