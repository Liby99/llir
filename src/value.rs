use std::marker::PhantomData;
use llvm_sys::core::{LLVMGetNextBasicBlock, LLVMGetFirstBasicBlock, LLVMGetOperand, LLVMValueAsBasicBlock};
use llvm_sys::prelude::{LLVMValueRef, LLVMBasicBlockRef};
use llvm_sys::LLVMOpcode;

#[derive(Copy, Clone)]
pub struct Function<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> Function<'ctx> {
  pub fn is_declaration_only(&self) -> bool {
    let first_block = unsafe { LLVMGetFirstBasicBlock(self.0) };
    first_block.is_null()
  }

  pub fn iter_blocks(&self) -> FunctionBlockIterator<'ctx> {
    let first_block = unsafe { LLVMGetFirstBasicBlock(self.0) };
    if first_block.is_null() { FunctionBlockIterator { curr_block: None } }
    else { FunctionBlockIterator { curr_block: Some(Block(first_block, PhantomData)) }}
  }
}

#[derive(Copy, Clone)]
pub struct Block<'ctx>(LLVMBasicBlockRef, PhantomData<&'ctx ()>);

pub struct FunctionBlockIterator<'ctx> {
  curr_block: Option<Block<'ctx>>,
}

impl<'ctx> Iterator for FunctionBlockIterator<'ctx> {
  type Item = Block<'ctx>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.curr_block {
      Some(block) => {
        let result = Some(block);
        let next_block_ptr = unsafe { LLVMGetNextBasicBlock(block.0) };
        if next_block_ptr.is_null() {
          self.curr_block = None;
        } else {
          self.curr_block = Some(Block(next_block_ptr, PhantomData));
        }
        result
      },
      None => None
    }
  }
}

#[derive(Copy, Clone)]
pub enum Instruction<'ctx> {
  Binary(BinaryInstruction<'ctx>),
  Unary(UnaryInstruction<'ctx>),
  Call(CallInstruction<'ctx>),
}

#[derive(Copy, Clone)]
pub enum Constant<'ctx> {
  Int(IntConstant<'ctx>),
  Float(FloatConstant<'ctx>),
  Null(NullConstant<'ctx>),
}

#[derive(Copy, Clone)]
pub struct IntConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

#[derive(Copy, Clone)]
pub struct FloatConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

#[derive(Copy, Clone)]
pub struct NullConstant<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

#[derive(Copy, Clone)]
pub enum Operand<'ctx> {
  Instruction(Instruction<'ctx>),
  Constant(Constant<'ctx>),
  Metadata, // TODO
}

#[derive(Copy, Clone)]
pub struct BinaryInstruction<'ctx>(BinaryOpcode, LLVMValueRef, PhantomData<&'ctx ()>);

#[derive(Copy, Clone)]
pub enum BinaryOpcode {
  // Arithmatics
  Add,
  Sub,
  Mul,
  UDiv,
  SDiv,
  URem,
  SRem,
  // Floating point
  FAdd,
  FSub,
  FMul,
  FDiv,
  FRem,
  // Bitwise operation
  Shl,
  LShr,
  AShr,
  And,
  Or,
  Xor,
  // Comparison
  ICmp,
  FCmp,
}

impl BinaryOpcode {
  pub fn from_llvm(llvm_opcode: LLVMOpcode) -> Option<Self> {
    match llvm_opcode {
      LLVMOpcode::LLVMAdd => Some(Self::Add),
      LLVMOpcode::LLVMSub => Some(Self::Sub),
      LLVMOpcode::LLVMMul => Some(Self::Mul),
      LLVMOpcode::LLVMUDiv => Some(Self::UDiv),
      LLVMOpcode::LLVMSDiv => Some(Self::SDiv),
      LLVMOpcode::LLVMURem => Some(Self::URem),
      LLVMOpcode::LLVMSRem => Some(Self::SRem),
      LLVMOpcode::LLVMFAdd => Some(Self::FAdd),
      LLVMOpcode::LLVMFSub => Some(Self::FSub),
      LLVMOpcode::LLVMFMul => Some(Self::FMul),
      LLVMOpcode::LLVMFDiv => Some(Self::FDiv),
      LLVMOpcode::LLVMShl => Some(Self::Shl),
      LLVMOpcode::LLVMLShr => Some(Self::LShr),
      LLVMOpcode::LLVMAShr => Some(Self::AShr),
      LLVMOpcode::LLVMAnd => Some(Self::And),
      LLVMOpcode::LLVMOr => Some(Self::Or),
      LLVMOpcode::LLVMXor => Some(Self::Xor),
      LLVMOpcode::LLVMICmp => Some(Self::ICmp),
      LLVMOpcode::LLVMFCmp => Some(Self::FCmp),
      _ => None
    }
  }
}

impl<'ctx> BinaryInstruction<'ctx> {
  pub fn opcode(&self) -> BinaryOpcode {
    self.0
  }

  pub fn op0(&self) -> Operand<'ctx> {
    // TODO
    Operand::Metadata
  }

  pub fn op1(&self) -> Operand<'ctx> {
    // TODO
    Operand::Metadata
  }
}

#[derive(Copy, Clone)]
pub struct UnaryInstruction<'ctx>(UnaryOpcode, LLVMValueRef, PhantomData<&'ctx ()>);

#[derive(Copy, Clone)]
pub enum UnaryOpcode {
  Trunc,
  ZExt,
  SExt,
  FPToUI,
  FPToSI,
  UIToFP,
  SIToFP,
  FPTrunc,
  FPExt,
  PtrToInt,
  IntToPtr,
  BitCast,
}

impl UnaryOpcode {
  pub fn from_llvm(llvm_opcode: LLVMOpcode) -> Option<Self> {
    match llvm_opcode {
      LLVMOpcode::LLVMTrunc => Some(Self::Trunc),
      LLVMOpcode::LLVMZExt => Some(Self::ZExt),
      LLVMOpcode::LLVMSExt => Some(Self::SExt),
      LLVMOpcode::LLVMFPToUI => Some(Self::FPToUI),
      LLVMOpcode::LLVMFPToSI => Some(Self::FPToSI),
      LLVMOpcode::LLVMUIToFP => Some(Self::UIToFP),
      LLVMOpcode::LLVMSIToFP => Some(Self::SIToFP),
      LLVMOpcode::LLVMFPTrunc => Some(Self::FPTrunc),
      LLVMOpcode::LLVMFPExt => Some(Self::FPExt),
      LLVMOpcode::LLVMPtrToInt => Some(Self::PtrToInt),
      LLVMOpcode::LLVMIntToPtr => Some(Self::IntToPtr),
      LLVMOpcode::LLVMBitCast => Some(Self::BitCast),
      _ => None
    }
  }
}

impl<'ctx> UnaryInstruction<'ctx> {
  pub fn opcode(&self) -> UnaryOpcode {
    self.0
  }

  pub fn op0(&self) -> Operand<'ctx> {
    // TODO
    Operand::Metadata
  }
}

#[derive(Copy, Clone)]
pub struct CallInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> CallInstruction<'ctx> {
  pub fn callee_function(&self) -> Option<Function<'ctx>> {
    // TODO
    None
  }

  pub fn callee(&self) -> Operand<'ctx> {
    // TODO
    Operand::Metadata
  }

  pub fn num_arguments(&self) -> usize {
    // TODO
    0
  }

  pub fn args(&self) -> Vec<Operand<'ctx>> {
    // TODO
    vec![]
  }
}

#[derive(Copy, Clone)]
pub struct ConditionalBranchInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> ConditionalBranchInstruction<'ctx> {
  pub fn condition(&self) -> Operand<'ctx> {
    // TODO
    Operand::Metadata
  }

  pub fn then_block(&self) -> Block<'ctx> {
    let operand = unsafe { LLVMGetOperand(self.0, 2) };
    let block = unsafe { LLVMValueAsBasicBlock(operand) };
    Block(block, PhantomData)
  }

  pub fn else_block(&self) -> Block<'ctx> {
    let operand = unsafe { LLVMGetOperand(self.0, 1) };
    let block = unsafe { LLVMValueAsBasicBlock(operand) };
    Block(block, PhantomData)
  }
}

#[derive(Copy, Clone)]
pub struct UnconditionalBranchInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> UnconditionalBranchInstruction<'ctx> {
  pub fn to_block(&self) -> Block<'ctx> {
    let operand = unsafe { LLVMGetOperand(self.0, 0) };
    let block = unsafe { LLVMValueAsBasicBlock(operand) };
    Block(block, PhantomData)
  }
}

#[derive(Copy, Clone)]
pub struct SwitchInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> SwitchInstruction<'ctx> {
  pub fn condition(&self) -> Operand<'ctx> {
    // TODO
    Operand::Metadata
  }

  pub fn default_block(&self) -> Block<'ctx> {
    let operand = unsafe { LLVMGetOperand(self.0, 1) };
    let block = unsafe { LLVMValueAsBasicBlock(operand) };
    Block(block, PhantomData)
  }

  pub fn num_branches(&self) -> usize {
    // TODO
    0
  }

  pub fn branches(&self) -> Vec<(Constant<'ctx>, Block<'ctx>)> {
    // TODO
    vec![]
  }
}

#[derive(Copy, Clone)]
pub struct ReturnInstruction<'ctx>(LLVMValueRef, PhantomData<&'ctx ()>);

impl<'ctx> ReturnInstruction<'ctx> {
  pub fn has_op(&self) -> bool {
    // TODO
    false
  }

  pub fn op(&self) -> Option<Operand<'ctx>> {
    // TODO
    None
  }
}