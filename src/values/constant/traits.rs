use crate::values::*;

/// Turn constant subclass into a constant enum
pub trait AsConstant<'ctx> {
  fn as_constant(&self) -> Constant<'ctx>;
}
