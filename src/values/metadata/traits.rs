use super::*;

pub trait AsMetadata<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx>;
}

// impl<'ctx, V> AsOperand<'ctx> for V where V: AsMetadata<'ctx> {
//   fn as_operand(&self) -> Operand<'ctx> {
//     Operand::Metadata(self.as_constant())
//   }
// }
