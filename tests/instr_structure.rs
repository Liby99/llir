use llir;
use std::path::Path;

fn test_no_crash<'ctx>(module: &llir::Module<'ctx>) {
  for func in module.iter_functions() {
    for block in func.iter_blocks() {
      for instr in block.iter_instructions() {
        use llir::values::Instruction::*;
        use llir::values::*;
        match instr {
          Binary(bin) => {
            let _ = bin.opcode();
            let _ = bin.op0();
            let _ = bin.op1();
          }
          Unary(una) => {
            let _ = una.opcode();
            let _ = una.op0();
          }
          Call(call) => {
            let _ = call.callee_function();
            let _ = call.callee();
            let _ = call.num_arguments();
            let _ = call.arguments();
            let _ = call.is_tail_call();
          }
          Branch(br) => match br {
            BranchInstruction::Conditional(cond_br) => {
              let _ = cond_br.condition();
              let _ = cond_br.then_block();
              let _ = cond_br.else_block();
            }
            BranchInstruction::Unconditional(uncond_br) => {
              let _ = uncond_br.target_block();
            }
          },
          Switch(switch) => {
            let _ = switch.condition();
            let _ = switch.default_block();
            let _ = switch.num_cases();
            let _ = switch.cases();
          }
          _ => (),
        }
      }
    }
  }
}

#[test]
fn test_instr_struct_free_struct() -> Result<(), String> {
  let path = Path::new("tests/c_files/free/free_struct.bc");
  let context = llir::Context::create();
  let module = context.load_module(path)?;
  test_no_crash(&module);
  Ok(())
}

#[test]
fn test_load_fn_ptr_1() -> Result<(), String> {
  let path = Path::new("tests/c_files/fn_ptr/fn_ptr_1.bc");
  let context = llir::Context::create();
  let module = context.load_module(path)?;
  test_no_crash(&module);
  Ok(())
}
