macro_rules! impl_instr_debug {
  ($id:ident) => {
    impl<'ctx> std::fmt::Debug for $id<'ctx> {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!($id)).field(&self.to_string()).finish()
      }
    }
  };
}

macro_rules! impl_as_operand_for_instr {
  ($id:ident) => {
    impl<'ctx> AsOperand<'ctx> for $id<'ctx> {
      fn as_operand(&self) -> Operand<'ctx> {
        Operand::Instruction(self.as_instruction())
      }
    }
  }
}

macro_rules! impl_op_from_llvm_value {
  ($id:ident, $op:ident, $op_getter:ident) => {
    impl<'ctx> FromLLVMValue for $id<'ctx> {
      fn from_llvm(ptr: LLVMValueRef) -> Self {
        let op = $op::from_llvm(unsafe { $op_getter(ptr) }).unwrap();
        Self(op, ptr, PhantomData)
      }
    }
  }
}

macro_rules! impl_cmp_from_llvm_value {
  ($id:ident, $op:ident, $op_getter:ident) => {
    impl<'ctx> FromLLVMValue for $id<'ctx> {
      fn from_llvm(ptr: LLVMValueRef) -> Self {
        let op = $op::from_llvm(unsafe { $op_getter(ptr) });
        Self(op, ptr, PhantomData)
      }
    }
  }
}