use super::*;

/// Turn metadata subclass into a Metadata enum
pub trait AsMetadata<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx>;
}

macro_rules! impl_as_operand_for_metadata {
  ($id:ident) => {
    impl<'ctx> AsOperand<'ctx> for $id<'ctx> {
      fn as_operand(&self) -> Operand<'ctx> {
        Operand::Metadata(self.as_metadata())
      }
    }
  }
}
