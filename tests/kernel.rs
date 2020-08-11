use std::path::Path;
use llir::*;
use llir::values::*;

#[test]
fn test_kernel_globals() -> Result<(), String> {
  let kernel_path = Path::new("/home/aspire/programs/linux_kernel/linux-4.5-rc4/vmlinux.bc");
  let ctx = Context::create();
  let module = ctx.load_module(kernel_path)?;
  for glob in module.iter_globals() {
    println!("{}: is_alias: {}", glob.name(), glob.is_alias());
  }
  // for func in module.iter_functions() {

  // }
  Ok(())
}