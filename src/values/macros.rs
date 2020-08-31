macro_rules! impl_positional_value_ref {
  ($id:ident, $pos:tt) => {
    impl<'ctx> ValueRef for $id<'ctx> {
      fn value_ref(&self) -> LLVMValueRef {
        self.$pos
      }
    }
  }
}

macro_rules! impl_positional_from_llvm_value {
  ($id:ident) => {
    impl<'ctx> FromLLVMValue for $id<'ctx> {
      fn from_llvm(ptr: LLVMValueRef) -> Self {
        Self(ptr, PhantomData)
      }
    }
  }
}