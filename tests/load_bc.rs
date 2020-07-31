use llir;
use std::path::Path;

#[test]
fn test_load_free_struct() -> Result<(), String> {
  let path = Path::new("tests/c_files/free/free_struct.bc");
  let context = llir::Context::create();
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

#[test]
fn test_load_fn_ptr_1() -> Result<(), String> {
  let path = Path::new("tests/c_files/fn_ptr/fn_ptr_1.bc");
  let context = llir::Context::create();
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
