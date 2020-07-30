use super::instruction::Instruction;
use super::constant::Constant;

#[derive(Copy, Clone)]
pub enum Operand<'ctx> {
  Instruction(Instruction<'ctx>),
  Constant(Constant<'ctx>),
  Metadata, // TODO
}