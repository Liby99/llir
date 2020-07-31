use llvm_sys::prelude::{LLVMTypeRef};
use std::marker::PhantomData;

pub struct VoidType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);