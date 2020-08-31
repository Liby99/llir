macro_rules! impl_positional_type_ref {
  ($id:ident, $pos:tt) => {
    impl<'ctx> TypeRef for $id<'ctx> {
      fn type_ref(&self) -> LLVMTypeRef {
        self.$pos
      }
    }
  }
}

macro_rules! impl_positional_from_llvm_type {
  ($id:ident) => {
    impl<'ctx> FromLLVMType for $id<'ctx> {
      fn from_llvm(ptr: LLVMTypeRef) -> Self {
        Self(ptr, PhantomData)
      }
    }
  }
}