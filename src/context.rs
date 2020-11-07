#[allow(deprecated)]
use llvm_sys::bit_reader::LLVMParseBitcodeInContext;
use llvm_sys::core::*;
use llvm_sys::prelude::LLVMContextRef;
use std::mem::MaybeUninit;
use std::path::Path;

use super::utils::mem_buffer::MemoryBuffer;
use super::utils::support::LLVMString;
use super::Module;

/// LLVM Context
///
/// ```
/// # use llir::Context;
/// let ctx = Context::create();
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Context(LLVMContextRef);

unsafe impl Send for Context {}

unsafe impl Sync for Context {}

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
  /// # use llir::Context;
  /// # let ctx = Context::create();
  /// let module = ctx.load_module(Path::new("tests/c_files/basic/example_1.bc"))?;
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

impl Drop for Context {
  fn drop(&mut self) {
    unsafe { LLVMContextDispose(self.0) }
  }
}
