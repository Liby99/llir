use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use std::ffi::CStr;

pub fn string_of_value(ptr: LLVMValueRef) -> String {
  let mut len = 0;
  let ptr = unsafe { LLVMGetValueName2(ptr, &mut len) };
  let cstr = unsafe { CStr::from_ptr(ptr) };
  cstr.to_str().expect("Failed to convert CStr").to_owned()
}