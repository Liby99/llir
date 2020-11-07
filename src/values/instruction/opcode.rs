use super::binary::BinaryOpcode;
use super::unary::UnaryOpcode;

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
  Alloca,
  Binary(BinaryOpcode),
  Br,
  CallBr,
  Call,
  ExtractValue,
  FCmp,
  GetElementPtr,
  ICmp,
  IndirectBr,
  InsertValue,
  Load,
  Phi,
  Ret,
  Select,
  Store,
  Switch,
  Unary(UnaryOpcode),
  Unreachable,
  Unknown,
}

impl Opcode {
  pub fn to_string(&self) -> &str {
    match self {
      Self::Alloca => "alloca",
      Self::Binary(bin_op) => bin_op.to_string(),
      Self::Br => "br",
      Self::CallBr => "callbr",
      Self::Call => "call",
      Self::ExtractValue => "extractvalue",
      Self::FCmp => "fcmp",
      Self::GetElementPtr => "getelementptr",
      Self::ICmp => "icmp",
      Self::IndirectBr => "indirectbr",
      Self::InsertValue => "insertvalue",
      Self::Load => "load",
      Self::Phi => "phi",
      Self::Ret => "ret",
      Self::Select => "select",
      Self::Store => "store",
      Self::Switch => "switch",
      Self::Unary(una_op) => una_op.to_string(),
      Self::Unreachable => "unreachable",
      Self::Unknown => "unknown",
    }
  }
}