use llvm_sys::prelude::{LLVMBasicBlockRef, LLVMTypeRef, LLVMValueRef};

#[doc(hidden)]
pub trait ValueRef {
  fn value_ref(&self) -> LLVMValueRef;
}

#[doc(hidden)]
pub trait BlockRef {
  fn block_ref(&self) -> LLVMBasicBlockRef;
}

#[doc(hidden)]
pub trait TypeRef {
  fn type_ref(&self) -> LLVMTypeRef;
}

#[doc(hidden)]
pub trait FromLLVMValue: Sized {
  fn from_llvm(ptr: LLVMValueRef) -> Self;
}

#[doc(hidden)]
pub trait FromLLVMBlock: Sized {
  fn from_llvm(ptr: LLVMBasicBlockRef) -> Self;
}

#[doc(hidden)]
pub trait FromLLVMType: Sized {
  fn from_llvm(ptr: LLVMTypeRef) -> Self;
}
