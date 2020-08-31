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
