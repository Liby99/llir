use llvm_sys::prelude::LLVMValueRef;

pub(crate) trait ValueRef {
  fn value_ref(&self) -> LLVMValueRef;
}

pub(crate) trait FromLLVM: Sized {
  fn from_llvm(ptr: LLVMValueRef) -> Self;
}
