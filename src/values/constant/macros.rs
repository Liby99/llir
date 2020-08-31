macro_rules! impl_as_operand_for_constant {
  ($id:ident) => {
    impl<'ctx> AsOperand<'ctx> for $id<'ctx> {
      fn as_operand(&self) -> Operand<'ctx> {
        Operand::Constant(self.as_constant())
      }
    }
  }
}