use super::*;

/// Turn metadata subclass into a Metadata enum
pub trait AsMetadata<'ctx> {
  fn as_metadata(&self) -> Metadata<'ctx>;
}

