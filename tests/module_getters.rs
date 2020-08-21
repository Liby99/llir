use std::path::{Path};
use llir::*;

#[test]
fn get_function_1() -> Result<(), String> {
  let ctx = Context::create();
  let module = ctx.load_module(Path::new("tests/c_files/br/br_1.bc"))?;
  let _ = module.get_function("main").unwrap();
  Ok(())
}

#[test]
fn get_global_1() -> Result<(), String> {
  let ctx = Context::create();
  let module = ctx.load_module(Path::new("tests/c_files/lock/lock_with_global.bc"))?;
  let _ = module.get_global_variable("global_lock").unwrap();
  Ok(())
}