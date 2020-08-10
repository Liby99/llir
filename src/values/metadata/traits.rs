use super::*;

pub trait AsMetadata<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx>;
}