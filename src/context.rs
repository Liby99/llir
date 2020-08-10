#[allow(deprecated)]
use llvm_sys::bit_reader::LLVMParseBitcodeInContext;
use llvm_sys::core::LLVMContextCreate;
use llvm_sys::prelude::LLVMContextRef;
use std::path::Path;

use std::mem::MaybeUninit;

use super::utils::mem_buffer::MemoryBuffer;
use super::utils::support::LLVMString;
use super::Module;

/// LLVM Context
///
/// ```
/// let ctx = Context::create();
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Context(LLVMContextRef);

impl Context {
  pub(crate) fn new(context: LLVMContextRef) -> Self {
    Self(context)
  }

  /// Create a new context
  pub fn create() -> Self {
    let context = unsafe { LLVMContextCreate() };
    Self::new(context)
  }

  /// Load a module
  ///
  /// ```
  /// let module = ctx.load_module(Path::new("PATH/TO/YOUR/BYTECODE.bc"));
  /// ```
  pub fn load_module<'ctx, P>(&'ctx self, path: P) -> Result<Module<'ctx>, String>
  where
    P: AsRef<Path>,
  {
    let buffer = MemoryBuffer::create_from_file(path.as_ref())?;
    let mut module = MaybeUninit::uninit();
    let mut err_string = MaybeUninit::uninit();
    #[allow(deprecated)]
    let success = unsafe {
      LLVMParseBitcodeInContext(
        self.0,
        buffer.memory_buffer,
        module.as_mut_ptr(),
        err_string.as_mut_ptr(),
      )
    };
    if success != 0 {
      let err_string = unsafe { err_string.assume_init() };
      return Err(LLVMString::new(err_string).to_string());
    }

    let module = unsafe { module.assume_init() };

    Ok(Module::new(module))
  }
}
