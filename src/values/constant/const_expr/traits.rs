use llvm_sys::core::*;

use crate::*;
use crate::values::*;

pub trait ConstExprTrait<'ctx>: ValueRef {
  /// Get the number of operands used in this instruction
  fn num_operands(&self) -> usize {
    unsafe { LLVMGetNumOperands(self.value_ref()) as usize }
  }

  /// Get the operand at a given index
  fn operand(&self, index: usize) -> Option<Constant<'ctx>> {
    if index < self.num_operands() {
      Some(Constant::from_llvm(unsafe {
        LLVMGetOperand(self.value_ref(), index as u32)
      }))
    } else {
      None
    }
  }

  /// Get the string representation of the instruction
  fn to_string(&self) -> String {
    unsafe { crate::utils::raw_to_string(LLVMPrintValueToString(self.value_ref())) }
  }
}

/// Turn constant expression subclass into a ConstExpr enum
pub trait AsConstExpr<'ctx> {
  fn as_const_expr(&self) -> ConstExpr<'ctx>;
}
