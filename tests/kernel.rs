use llir::values::*;
use llir::types::*;
use llir::*;
use llvm_sys::core::*;
use std::path::Path;

fn test_global<'ctx>(glob: &Global<'ctx>) -> Result<(), String> {
  let _ = glob.name();
  let _ = glob.aliasee();
  let _ = glob.is_alias();
  Ok(())
}

fn test_pointer_type<'ctx>(p: &PointerType<'ctx>) -> Result<(), String> {
  Ok(())
}

fn test_function_type<'ctx>(t: &FunctionType<'ctx>) -> Result<(), String> {
  Ok(())
}

fn test_type<'ctx>(t: &Type<'ctx>) -> Result<(), String> {
  Ok(())
}

fn test_int_constant<'ctx>(i: &IntConstant<'ctx>) -> Result<(), String> {
  Ok(())
}

fn test_constant<'ctx>(c: &Constant<'ctx>) -> Result<(), String> {
  // match c {
  //   Constant
  // }
  Ok(())
}

fn test_operand<'ctx>(op: &Operand<'ctx>) -> Result<(), String> {
  use Operand::*;
  match op {
    Instruction(_) => {}
    Argument(arg) => {
      let _ = arg.parent();
      let _ = arg.index();
    }
    Constant(c) => {
      test_constant(c)?;
    }
    InlineAsm(_) => {}
    Metadata(_) => {}
  }
  Ok(())
}

fn test_instruction<'ctx>(instr: &Instruction<'ctx>) -> Result<(), String> {
  use Instruction::*;
  match instr {
    Alloca(a) => {
      test_pointer_type(&a.get_pointer_type())?;
    }
    Binary(b) => {
      let _ = b.opcode();
      test_operand(&b.op0())?;
      test_operand(&b.op1())?;
    }
    Branch(b) => {
      let _ = b.destinations();
      match b {
        BranchInstruction::Conditional(c) => {
          test_operand(&c.condition())?;
          let _ = c.then_block();
          let _ = c.else_block();
        },
        BranchInstruction::Unconditional(u) => {
          let _ = u.destination();
          let _ = u.is_loop_jump();
        }
      }
    }
    Call(c) => {
      let _ = c.callee_function();
      let _ = c.callee_inline_asm();
      test_function_type(&c.callee_function_type())?;
      let _ = c.num_arguments();
      let _ = c.arguments();
    }
    CallBr(_) => {}
    ExtractValue(ev) => {
      test_operand(&ev.aggregate())?;
      let _ = ev.num_indices();
      let _ = ev.indices();
    }
    FCmp(f) => {
      let _ = f.predicate();
      test_operand(&f.op0())?;
      test_operand(&f.op1())?;
    }
    GetElementPtr(g) => {
      test_operand(&g.location())?;
      let _ = g.num_indices();
      let _ = g.indices();
      test_pointer_type(&g.get_pointer_type())?;
    }
    IndirectBranch(ib) => {
      let _ = ib.address();
      let _ = ib.destinations();
    }
    ICmp(i) => {
      let _ = i.predicate();
      test_operand(&i.op0())?;
      test_operand(&i.op1())?;
    }
    InsertValue(iv) => {
      test_operand(&iv.aggregate())?;
      test_operand(&iv.value())?;
      let _ = iv.num_indices();
      let _ = iv.indices();
    }
    Load(l) => {
      test_operand(&l.location())?;
    }
    Phi(p) => {
      let _ = p.num_incomings();
      for inc in p.incomings() {
        test_operand(&inc.value)?;
      }
    }
    Return(r) => {
      if r.has_op() {
        test_operand(&r.op().unwrap())?;
      }
    }
    Select(s) => {
      test_operand(&s.condition())?;
      test_operand(&s.true_value())?;
      test_operand(&s.false_value())?;
    }
    Store(s) => {
      test_operand(&s.location())?;
      test_operand(&s.value())?;
    }
    Switch(s) => {
      test_operand(&s.condition())?;
      let _ = s.default_destination();
      let _ = s.num_cases();
      for case in s.cases() {
        test_int_constant(&case.case)?;
      }
    }
    Unary(u) => {
      let _ = u.opcode();
      test_operand(&u.op0())?;
    }
    Unreachable(_) => {}
    Other(o) => {
      println!("Other instruction: {:?}", unsafe {
        LLVMGetInstructionOpcode(o.value_ref())
      });
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
