use crate::values::*;

/// Turn constant subclass into a constant enum
pub trait AsConstant<'ctx> {
  fn as_constant(&self) -> Constant<'ctx>;
}

// impl<'ctx, V> AsOperand<'ctx> for V where V: AsConstant<'ctx> {
//   fn as_operand(&self) -> Operand<'ctx> {
//     Operand::Constant(self.as_constant())
//   }
// }
