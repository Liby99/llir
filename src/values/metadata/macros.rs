macro_rules! impl_as_operand_for_metadata {
  ($id:ident) => {
    impl<'ctx> AsOperand<'ctx> for $id<'ctx> {
      fn as_operand(&self) -> Operand<'ctx> {
        Operand::Metadata(self.as_metadata())
      }
    }
  }
}
