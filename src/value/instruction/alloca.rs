use std::marker::PhantomData;
use llvm_sys::prelude::LLVMValueRef;

#[derive(Copy, Clone)]
pub struct AllocaInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> AllocaInstruction<'ctx> {

}