use llir;
use llir::values::*;
use std::path::Path;

#[test]
fn test_intrinsic_call() -> Result<(), String> {
  let path = Path::new("tests/c_files/br/br_1.bc");
  let context = llir::Context::create();
  let module = context.load_module(path)?;
  for func in module.iter_functions() {
    for block in func.iter_blocks() {
      for instr in block.iter_instructions() {
        match instr {
          Instruction::Call(c) => {
            println!("Instr {} is intrinsic: {}", c.to_string(), c.is_intrinsic_call());
          }
          _ => {}
        }
      }
    }
  }
  Ok(())
}
