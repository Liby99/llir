use super::*;

#[derive(Copy, Clone)]
pub enum Instruction<'ctx> {
  Binary(BinaryInstruction<'ctx>),
  Unary(UnaryInstruction<'ctx>),
  Call(CallInstruction<'ctx>),
  ConditionalBranch(ConditionalBranchInstruction<'ctx>),
  UnconditionalBranch(UnconditionalBranchInstruction<'ctx>),
  Switch(SwitchInstruction<'ctx>),
  Return(ReturnInstruction<'ctx>),
  Alloca(AllocaInstruction<'ctx>),
  Load(LoadInstruction<'ctx>),
  Store(StoreInstruction<'ctx>),
}