use llir;
use std::path::Path;

#[test]
fn fn_ptr_1_block_name() -> Result<(), String> {
  let path = Path::new("tests/c_files/fn_ptr/fn_ptr_1.bc");
  let context = llir::Context::create();
  let module = context.load_module(path)?;
  for func in module.iter_functions() {
    for block in func.iter_blocks() {
      println!("{}", block.name());
    }
  }
  Ok(())
}
