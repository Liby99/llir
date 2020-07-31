use llir;
use llir::values::*;
use std::path::Path;

#[test]
fn test_debug_loc() -> Result<(), String> {
  let path = Path::new("tests/c_files/free/free_struct.bc");
  let context = llir::Context::create();
  let module = context.load_module(path)?;
  for func in module.iter_functions() {
    println!("Func Loc: {}", func.debug_loc_string());
    for block in func.iter_blocks() {
      for instr in block.iter_instructions() {
        println!("Instr Loc: {}", instr.debug_loc_string());
      }
    }
  }
  Ok(())
}
