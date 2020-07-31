use llvm;
use std::path::Path;

#[test]
fn test_load_free_struct() -> Result<(), String> {
  let path = Path::new("tests/c_files/free/free_struct.bc");
  let context = llvm::Context::create();
  let module = context.load_module(path)?;
  for func in module.iter_functions() {
    println!("{}, {}", func.name(), func.is_declaration_only());
    for block in func.iter_blocks() {
      for instr in block.iter_instructions() {
        println!("{:?}", instr);
      }
    }
  }
  Ok(())
}