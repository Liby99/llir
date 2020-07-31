use llir;
use std::path::Path;

#[test]
fn test_function_pointer_1() -> Result<(), String> {
  let path = Path::new("tests/c_files/fn_ptr/fn_ptr_1.bc");
  let context = llir::Context::create();
  let module = context.load_module(path)?;
  for func in module.iter_functions() {
    for block in func.iter_blocks() {
      for instr in block.iter_instructions() {
        use llir::values::Instruction;
        match instr {
          Instruction::Call(call) => {
            let f = call.callee_function();
            match f {
              Some(f) => println!("{}", f.name()),
              None => println!("Calling function pointer"),
            }
          }
          _ => {}
        }
      }
    }
  }
  Ok(())
}
