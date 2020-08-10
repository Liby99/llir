use llvm_sys::prelude::{LLVMBasicBlockRef, LLVMTypeRef, LLVMValueRef};

pub trait ValueRef {
  fn value_ref(&self) -> LLVMValueRef;
}

pub trait BlockRef {
  fn block_ref(&self) -> LLVMBasicBlockRef;
}

pub trait TypeRef {
  fn type_ref(&self) -> LLVMTypeRef;
}

pub(crate) trait FromLLVMValue: Sized {
  fn from_llvm(ptr: LLVMValueRef) -> Self;
}

pub(crate) trait FromLLVMBlock: Sized {
  fn from_llvm(ptr: LLVMBasicBlockRef) -> Self;
}

pub(crate) trait FromLLVMType: Sized {
  fn from_llvm(ptr: LLVMTypeRef) -> Self;
}
