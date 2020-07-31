use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::ffi::CStr;
use std::os::raw::c_char;

pub unsafe fn raw_to_string(raw: *const c_char) -> String {
  let cstr = CStr::from_ptr(raw);
  cstr.to_str().expect("Failed to convert CStr").to_owned()
}

pub fn string_of_value(ptr: LLVMValueRef) -> String {
  let mut len = 0;
  let ptr = unsafe { LLVMGetValueName2(ptr, &mut len) };
  unsafe { raw_to_string(ptr) }
}

pub fn string_of_type(ptr: LLVMTypeRef) -> String {
  let ptr = unsafe { LLVMGetStructName(ptr) };
  unsafe { raw_to_string(ptr) }
}

pub fn string_of_debugloc_filename(ptr: LLVMValueRef) -> Option<String> {
  let mut len = 0;
  let ptr = unsafe { LLVMGetDebugLocFilename(ptr, &mut len) };
  if len == 0 || ptr.is_null() {
    None
  } else {
    Some(unsafe { raw_to_string(ptr) })
  }
}

pub fn string_of_debugloc_directory(ptr: LLVMValueRef) -> Option<String> {
  let mut len = 0;
  let ptr = unsafe { LLVMGetDebugLocDirectory(ptr, &mut len) };
  if len == 0 || ptr.is_null() {
    None
  } else {
    Some(unsafe { raw_to_string(ptr) })
  }
}
