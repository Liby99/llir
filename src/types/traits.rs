use super::*;

/// Turn every type's child classes into a type container enum
pub trait AsType<'ctx> {
  fn as_type(&self) -> Type<'ctx>;
}
