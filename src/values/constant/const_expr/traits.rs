use crate::values::*;

/// Turn constant expression subclass into a ConstExpr enum
pub trait AsConstExpr<'ctx> {
  fn as_const_expr(&self) -> ConstExpr<'ctx>;
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