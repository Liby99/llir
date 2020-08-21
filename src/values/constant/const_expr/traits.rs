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

macro_rules! impl_const_expr_debug {
  ($id:ident) => {
    impl<'ctx> std::fmt::Debug for $id<'ctx> {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!($id)).field(&self.to_string()).finish()
      }
    }
  };
}

macro_rules! impl_as_constant_and_as_operand_for_const_expr {
  ($id:ident) => {
    impl<'ctx> AsConstant<'ctx> for $id<'ctx> {
      fn as_constant(&self) -> Constant<'ctx> {
        Constant::ConstExpr(self.as_const_expr())
      }
    }

    impl<'ctx> AsOperand<'ctx> for $id<'ctx> {
      fn as_operand(&self) -> Operand<'ctx> {
        Operand::Constant(self.as_constant())
      }
    }
  }
}