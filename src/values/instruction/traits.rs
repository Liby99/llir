use super::Instruction;

pub trait AsInstruction<'ctx> {
  fn as_instruction(&self) -> Instruction<'ctx>;
}