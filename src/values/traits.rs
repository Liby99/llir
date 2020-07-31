use llvm_sys::core::{LLVMGetDebugLocColumn, LLVMGetDebugLocLine, LLVMTypeOf};

use crate::types::*;
use crate::utils::*;
use crate::*;

pub trait HasType {}

pub trait GetType<'ctx> {
  fn get_type(&self) -> Type<'ctx>;
}

impl<'ctx, V> GetType<'ctx> for V
where
  V: ValueRef + HasType,
{
  fn get_type(&self) -> Type<'ctx> {
    Type::from_llvm(unsafe { LLVMTypeOf(self.value_ref()) })
  }
}

pub trait HasDebugLoc {}

pub trait DebugLoc {
  fn filename(&self) -> Option<String>;

  fn directory(&self) -> Option<String>;

  fn line(&self) -> Option<u32>;

  fn col(&self) -> Option<u32>;

  fn debug_loc_string(&self) -> String {
    match (self.directory(), self.filename()) {
      (Some(dir), Some(file)) => match (self.line(), self.col()) {
        (Some(line), Some(col)) => return format!("{}/{}:{}:{}", dir, file, line, col),
        _ => {}
      },
      _ => {}
    };
    String::from("0:0")
  }
}

impl<'ctx, V> DebugLoc for V
where
  V: HasDebugLoc + ValueRef,
{
  fn filename(&self) -> Option<String> {
    string_of_debugloc_filename(self.value_ref())
  }

  fn directory(&self) -> Option<String> {
    string_of_debugloc_directory(self.value_ref())
  }

  fn line(&self) -> Option<u32> {
    Some(unsafe { LLVMGetDebugLocLine(self.value_ref()) })
  }

  fn col(&self) -> Option<u32> {
    Some(unsafe { LLVMGetDebugLocColumn(self.value_ref()) })
  }
}
