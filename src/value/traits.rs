use llvm_sys::prelude::LLVMValueRef;

pub(crate) trait ValueRef {
  fn value_ref(&self) -> LLVMValueRef;
}
