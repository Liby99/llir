use llvm_sys::core::{LLVMCreateMemoryBufferWithContentsOfFile, LLVMDisposeMemoryBuffer};
use llvm_sys::prelude::LLVMMemoryBufferRef;

use super::support::{to_c_str, LLVMString};

use std::mem::MaybeUninit;
use std::path::Path;
use std::ptr;

#[derive(Debug)]
pub struct MemoryBuffer {
  pub(crate) memory_buffer: LLVMMemoryBufferRef,
}

impl MemoryBuffer {
  pub(crate) fn new(memory_buffer: LLVMMemoryBufferRef) -> Self {
    assert!(!memory_buffer.is_null());

    MemoryBuffer { memory_buffer }
  }

  pub fn create_from_file(path: &Path) -> Result<Self, String> {
    let path = to_c_str(path.to_str().expect("Did not find a valid Unicode path string"));
    let mut memory_buffer = ptr::null_mut();
    let mut err_string = MaybeUninit::uninit();

    let return_code = unsafe {
      LLVMCreateMemoryBufferWithContentsOfFile(
        path.as_ptr() as *const ::libc::c_char,
        &mut memory_buffer,
        err_string.as_mut_ptr(),
      )
    };

    if return_code == 1 {
      let err_str = unsafe { err_string.assume_init() };
      return Err(LLVMString::new(err_str).to_string());
    }

    Ok(MemoryBuffer::new(memory_buffer))
  }
}

impl Drop for MemoryBuffer {
  fn drop(&mut self) {
    unsafe {
      LLVMDisposeMemoryBuffer(self.memory_buffer);
    }
  }
}
