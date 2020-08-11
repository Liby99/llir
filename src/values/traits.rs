use llvm_sys::core::{LLVMGetDebugLocColumn, LLVMGetDebugLocLine, LLVMTypeOf};

use super::*;
use crate::types::*;
use crate::utils::*;
use crate::*;

/// GetType trait provides `get_type` functions
pub trait GetType<'ctx>: ValueRef {
  fn get_type(&self) -> Type<'ctx> {
    Type::from_llvm(unsafe { LLVMTypeOf(self.value_ref()) })
  }
}

/// Get debug metadata for any value
pub trait GetDebugMetadata<'ctx>: ValueRef {
  fn dbg_metadata(&self) -> Option<Metadata<'ctx>> {
    mdkind_ids::dbg_metadata(self.value_ref()).map(Metadata::from_llvm)
  }
}

/// Trait that turns a value into an operand
pub trait AsOperand<'ctx> {
  fn as_operand(&self) -> Operand<'ctx>;
}

/// InstructionDebugLoc is implemented whenever it is an instruction
pub trait InstructionDebugLoc {}

/// GetDebugLoc trait is implemented when there's debug location with respect
/// to a value
pub trait GetDebugLoc {
  fn filename(&self) -> Option<String>;

  fn line(&self) -> Option<u32>;

  fn col(&self) -> Option<u32>;

  fn debug_loc_string(&self) -> String {
    match self.filename() {
      Some(file) => match (self.line(), self.col()) {
        (Some(line), Some(col)) => return format!("{}:{}:{}", file, line, col),
        _ => return file,
      },
      _ => {}
    };
    String::new()
  }
}

impl<'ctx, V> GetDebugLoc for V
where
  V: InstructionDebugLoc + ValueRef,
{
  fn filename(&self) -> Option<String> {
    match (
      string_of_debugloc_directory(self.value_ref()),
      string_of_debugloc_filename(self.value_ref()),
    ) {
      (Some(dir), Some(file)) => Some(format!("{}/{}", dir, file)),
      _ => None,
    }
  }

  fn line(&self) -> Option<u32> {
    Some(unsafe { LLVMGetDebugLocLine(self.value_ref()) })
  }

  fn col(&self) -> Option<u32> {
    Some(unsafe { LLVMGetDebugLocColumn(self.value_ref()) })
  }
}

/// Turn the value into a GenericValue, implemented for every value class
pub trait AsGenericValue<'ctx>: ValueRef {
  fn as_generic_value(&self) -> GenericValue<'ctx> {
    GenericValue::new(self.value_ref())
  }
}

impl<'ctx, V> AsGenericValue<'ctx> for V where V: ValueRef {}
