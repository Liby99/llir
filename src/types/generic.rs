use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

use crate::{FromLLVMType, TypeRef};

/// A placeholder type; used when the type is not supported yet
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GenericType<'ctx>(LLVMTypeRef, PhantomData<&'ctx ()>);

unsafe impl<'ctx> Send for GenericType<'ctx> {}

unsafe impl<'ctx> Sync for GenericType<'ctx> {}

impl_positional_type_ref!(GenericType, 0);

impl_positional_from_llvm_type!(GenericType);