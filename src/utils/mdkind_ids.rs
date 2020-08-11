use libc::{c_char, c_uint};
use llvm_sys::core::*;
use llvm_sys::prelude::*;

pub fn mdkind_id(name: &str) -> c_uint {
  let len = name.len();
  let ptr = name.as_ptr();
  unsafe { LLVMGetMDKindID(ptr as *const c_char, len as c_uint) }
}

pub fn dbg_mdkind_id() -> c_uint {
  mdkind_id("dbg")
}

pub fn loop_mdkind_id() -> c_uint {
  mdkind_id("llvm.loop")
}

pub fn dbg_metadata(val: LLVMValueRef) -> Option<LLVMValueRef> {
  let ptr = unsafe { LLVMGetMetadata(val, dbg_mdkind_id()) };
  if ptr.is_null() {
    None
  } else {
    Some(ptr)
  }
}

pub fn loop_metadata(val: LLVMValueRef) -> Option<LLVMValueRef> {
  let ptr = unsafe { LLVMGetMetadata(val, loop_mdkind_id()) };
  if ptr.is_null() {
    None
  } else {
    Some(ptr)
  }
}
