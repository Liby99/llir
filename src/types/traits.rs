use super::*;

pub trait AsType<'ctx> {
  fn as_type(&self) -> Type<'ctx>;
}
