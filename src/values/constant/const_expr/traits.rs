use crate::values::*;

/// Turn constant expression subclass into a ConstExpr enum
pub trait AsConstExpr<'ctx> {
  fn as_const_expr(&self) -> ConstExpr<'ctx>;
}

impl<'ctx, V> AsConstant<'ctx> for V
where
  V: AsConstExpr<'ctx>,
{
  fn as_constant(&self) -> Constant<'ctx> {
    Constant::ConstExpr(self.as_const_expr())
  }
}
