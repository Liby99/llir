use llir;
use llir::values::*;
use std::path::Path;

#[test]
fn dbgmd_1() -> Result<(), String> {
  let path = Path::new("tests/c_files/nested_loop/nested_loop_1.bc");
  let context = llir::Context::create();
  let module = context.load_module(path)?;
  for func in module.iter_functions() {
    for block in func.iter_blocks() {
      for instr in block.iter_instructions() {
        println!("{:?} - dbg mdnode: {:?}", instr, instr.dbg_metadata());
      }
    }
  }
  Ok(())
}

#[test]
fn loop_mdnode_1() -> Result<(), String> {
  let path = Path::new("tests/c_files/nested_loop/nested_loop_1.bc");
  let context = llir::Context::create();
  let module = context.load_module(path)?;
  for func in module.iter_functions() {
    for block in func.iter_blocks() {
      for instr in block.iter_instructions() {
        match instr {
          Instruction::Branch(BranchInstruction::Unconditional(uc)) => {
            println!("{:?} - is loop: {:?}", uc, uc.is_loop_jump());
          }
          _ => {}
        }
      }
    }
  }
  Ok(())
}