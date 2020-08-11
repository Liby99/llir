use std::path::Path;
use llir::*;
use llir::values::*;
use llvm_sys::core::*;
use llvm_sys::*;

fn test_global<'ctx>(glob: &Global<'ctx>) -> Result<(), String> {
  let _ = glob.name();
  let _ = glob.aliasee();
  let _ = glob.is_alias();
  Ok(())
}

fn test_instruction<'ctx>(instr: &Instruction<'ctx>) -> Result<(), String> {
  use Instruction::*;
  match instr {
    Alloca(a) => {

    }
    Binary(b) => {

    }
    Branch(b) => {

    }
    Call(c) => {

    }
    CallBr(c) => {

    }
    FCmp(f) => {

    }
    GetElementPtr(g) => {

    }
    ICmp(i) => {

    }
    Load(l) => {

    }
    Phi(p) => {

    }
    Return(r) => {

    }
    Select(_) => {

    }
    Store(s) => {

    }
    Switch(s) => {

    }
    Unary(u) => {

    }
    Unreachable(u) => {

    }
    Other(o) => {
      println!("Other instruction: {:?}", unsafe { LLVMGetInstructionOpcode(o.value_ref()) } );
    }
  }
  Ok(())
}

fn test_block<'ctx>(blk: &Block<'ctx>) -> Result<(), String> {
  let _ = blk.first_instruction();
  let _ = blk.last_instruction();
  for instr in blk.iter_instructions() {
    test_instruction(&instr)?;
  }
  Ok(())
}

fn test_func<'ctx>(func: &Function<'ctx>) -> Result<(), String> {
  let _ = func.name();
  let _ = func.is_declaration_only();
  let _ = func.first_block();
  let _ = func.last_block();
  let _ = func.is_var_arg();
  let _ = func.num_arguments();
  let _ = func.arguments();
  for blk in func.iter_blocks() {
    assert_eq!(blk.parent_function(), *func);
    test_block(&blk)?;
  }
  Ok(())
}

fn test_module<'ctx>(module: &Module<'ctx>) -> Result<(), String> {
  for glob in module.iter_globals() {
    test_global(&glob)?;
  }
  for func in module.iter_functions() {
    test_func(&func)?;
  }
  Ok(())
}

#[test]
fn test_kernel_globals() -> Result<(), String> {
  let kernel_path = Path::new("/home/aspire/programs/linux_kernel/linux-4.5-rc4/vmlinux.bc");
  let ctx = Context::create();
  let module = ctx.load_module(kernel_path)?;
  test_module(&module)
}