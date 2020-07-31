use libc::c_char;
use llvm_sys::core::LLVMDisposeMessage;

use std::borrow::Cow;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;

#[derive(Eq)]
pub struct LLVMString {
  pub(crate) ptr: *const c_char,
}

impl LLVMString {
  pub(crate) fn new(ptr: *const c_char) -> Self {
    LLVMString { ptr }
  }

  /// This is a convenience method for creating a Rust `String`,
  /// however; it *will* reallocate. `LLVMString` should be used
  /// as much as possible to save memory since it is allocated by
  /// LLVM. It's essentially a `CString` with a custom LLVM
  /// deallocator
  pub fn to_string(&self) -> String {
    (*self).to_string_lossy().into_owned()
  }
}

impl Deref for LLVMString {
  type Target = CStr;

  fn deref(&self) -> &Self::Target {
    unsafe { CStr::from_ptr(self.ptr) }
  }
}

impl Debug for LLVMString {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    write!(f, "{:?}", self.deref())
  }
}

impl Display for LLVMString {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    write!(f, "{:?}", self.deref())
  }
}

impl PartialEq for LLVMString {
  fn eq(&self, other: &LLVMString) -> bool {
    **self == **other
  }
}

impl Error for LLVMString {
  fn description(&self) -> &str {
    self
      .to_str()
      .expect("Could not convert LLVMString to str (likely invalid unicode)")
  }

  fn cause(&self) -> Option<&dyn Error> {
    None
  }
}

impl Drop for LLVMString {
  fn drop(&mut self) {
    unsafe {
      LLVMDisposeMessage(self.ptr as *mut _);
    }
  }
}

/// This function takes in a Rust string and either:
///
/// A) Finds a terminating null byte in the Rust string and can reference it directly like a C string.
///
/// B) Finds no null byte and allocates a new C string based on the input Rust string.
pub(crate) fn to_c_str<'s>(mut s: &'s str) -> Cow<'s, CStr> {
  if s.is_empty() {
    s = "\0";
  }

  // Start from the end of the string as it's the most likely place to find a null byte
  if s.chars().rev().find(|&ch| ch == '\0').is_none() {
    return Cow::from(CString::new(s).expect("unreachable since null bytes are checked"));
  }

  unsafe { Cow::from(CStr::from_ptr(s.as_ptr() as *const _)) }
}
