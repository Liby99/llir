use super::constant::Constant;
use super::instruction::Instruction;

#[derive(Copy, Clone)]
pub enum Operand<'ctx> {
  Instruction(Instruction<'ctx>),
  Constant(Constant<'ctx>),
  Metadata, // TODO
}
