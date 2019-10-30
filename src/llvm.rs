use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use std::collections::HashMap;

use crate::types::{
  context::Context as VarContext, func::Func, node::Node, opcode::Opcode,
  program::Program, variable::Variable,
};
use inkwell::basic_block::BasicBlock;
use inkwell::values::{FunctionValue, InstructionValue, IntValue, PointerValue};
use inkwell::IntPredicate;

/// Convenience type alias for the `sum` function.
///
/// Calling this is innately `unsafe` because there's no guarantee it doesn't
/// do `unsafe` operations internally.
type MainFunc = unsafe extern "C" fn() -> i32;

/// Compiler holds the LLVM state for the compilation
pub struct Compiler {
  pub context: Context,
  pub builder: Builder,
  pub module: Module,
  // pub fn_value_opt: Option<FunctionValue>,
  pub variables: HashMap<String, PointerValue>,
  pub funcs: HashMap<String, FunctionValue>,
}

impl Compiler {
  pub fn new() -> Self {
    let context = Context::create();
    Compiler {
      builder: context.create_builder(),
      module: context.create_module("main"),
      context: context,
      variables: HashMap::new(),
      funcs: HashMap::new(),
    }
  }

  fn get_variable(&self, id: &str) -> &PointerValue {
    match self.variables.get(id) {
      Some(var) => var,
      None => panic!(
        "Could not find a matching variable, {} in {:?}",
        id, self.variables
      ),
    }
  }

  fn compile_expr(&mut self, expr: &Node, funcs: &HashMap<String, Func>) -> IntValue {
    match expr {
      Node::Number(n) => return self.context.i32_type().const_int(*n as u64, false),
      Node::Var(name) => {
        let var = self.get_variable(&name);
        return self.builder.build_load(*var, &name).into_int_value();
      }
      Node::Op(left, op, right) => {
        let left_val = self.compile_expr(left, funcs);
        let right_val = self.compile_expr(right, funcs);
        return match op {
          Opcode::Add => left_val.const_add(right_val),
          Opcode::Sub => left_val.const_sub(right_val),
          Opcode::Mul => left_val.const_mul(right_val),
          Opcode::Div => left_val.const_signed_div(right_val),
          // Opcode::Eq => left_val.const_int_compare(IntPredicate::NE, right_val),
          Opcode::Eq => {
            self
              .builder
              .build_int_compare(IntPredicate::EQ, left_val, right_val, "eq")
          }
          // Opcode::Eq => left_val.const_add(right_val),
          // Opcode::Eq => self.context.i64_type().const_int(0, false),
          Opcode::Neq => left_val.const_int_compare(IntPredicate::NE, right_val),
          Opcode::Geq => left_val.const_int_compare(IntPredicate::SGE, right_val),
          Opcode::Leq => left_val.const_int_compare(IntPredicate::SLE, right_val),
          Opcode::Gneq => left_val.const_int_compare(IntPredicate::SGT, right_val),
          Opcode::Lneq => left_val.const_int_compare(IntPredicate::SLT, right_val),
          Opcode::And => left_val.const_and(right_val),
          Opcode::Or => left_val.const_or(right_val),
        };
      }
      _ => unreachable!(),
    };
  }

  pub fn compile_program(&mut self, program: &Program) -> Option<JitFunction<MainFunc>> {
    let execution_engine = self
      .module
      .create_jit_execution_engine(OptimizationLevel::None)
      .unwrap();

    for (_, func) in program.funcs.iter() {
      self.compile_func(func, &program.funcs);
    }

    let temp = unsafe { execution_engine.get_function("main").ok() };
    self.module.print_to_stderr();
    return temp;
  }

  fn compile_func(&mut self, func: &Func, funcs: &HashMap<String, Func>) {
    let i32_type = self.context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = self.module.add_function(&func.name, fn_type, None);
    // let basic_block = self.context.append_basic_block(&function, "entry");
    self.funcs.insert(func.name.to_string(), function);
    // self.builder.position_at_end(&basic_block);
    self.compile_block(&func.body_start, &func.name, &function, funcs);
  }

  /// Creates a new stack allocation instruction in the entry block of the function.
  fn create_entry_block_alloca(&mut self, function: &str, name: &str) -> PointerValue {
    let builder = self.context.create_builder();

    let entry = match self.funcs.get(function) {
      Some(func) => func.get_first_basic_block().unwrap(),
      None => panic!("Function not found"),
    };

    match entry.get_first_instruction() {
      Some(first_instr) => builder.position_before(&first_instr),
      None => builder.position_at_end(&entry),
    }
    let alloca = builder.build_alloca(self.context.i32_type(), name);
    self.variables.insert(name.to_string(), alloca);
    alloca
  }

  fn compile_node(
    &mut self,
    node: &Node,
    func: &FunctionValue,
    funcs: &HashMap<String, Func>,
  ) {
    match node {
      Node::Return(expr, _) => {
        println!("return");
        let expr_val = self.compile_expr(expr, funcs);
        self.builder.build_return(Some(&expr_val));
      }
      Node::Let(id, _, _, expr, _) => {
        println!("let");
        let expr_val = self.compile_expr(expr, funcs);
        let alloca = self.create_entry_block_alloca("main", id);
        self.builder.build_store(alloca, expr_val);
      }
      Node::If(condition, then_body, else_body, _) => {
        match else_body {
          Some(else_body) => {
            self.compile_if_else(condition, then_body, else_body, func, funcs)
          }
          None => self.compile_if(condition, then_body, func, funcs),
        };
      }
      Node::Empty => (),
      _ => unreachable!("{:#?}", node),
    };
  }

  fn compile_if_else(
    &mut self,
    condition: &Node,
    then_body: &Node,
    else_body: &Node,
    func: &FunctionValue,
    funcs: &HashMap<String, Func>,
  ) {
    let cond = self.compile_expr(condition, funcs);

    // build branch
    let then_block = self.context.append_basic_block(&func, "then");
    let else_block = self.context.append_basic_block(&func, "else");
    let cont_block = self.context.append_basic_block(&func, "cont");

    self
      .builder
      .build_conditional_branch(cond, &then_block, &else_block);

    // build then block
    self.builder.position_at_end(&then_block);
    self.compile_node(then_body, func, funcs);
    self.builder.build_unconditional_branch(&cont_block);

    // build else block
    self.builder.position_at_end(&else_block);
    self.compile_node(else_body, func, funcs);
    self.builder.build_unconditional_branch(&cont_block);

    // emit merge block
    self.builder.position_at_end(&cont_block);

    // let phi = self.builder.build_phi(self.context.i32_type(), "iftmp");

    // phi.add_incoming(&[(&then_val, &then_bb), (&else_val, &else_bb)]);

    // Ok(phi.as_basic_value().into_float_value());
  }
  fn compile_if(
    &mut self,
    condition: &Node,
    then_body: &Node,
    func: &FunctionValue,
    funcs: &HashMap<String, Func>,
  ) {
    let parent = func;

    let cond = self.compile_expr(condition, funcs);

    // build branch
    let then_block = self.context.append_basic_block(&parent, "then");
    let cont_block = self.context.append_basic_block(&parent, "cont");

    self
      .builder
      .build_conditional_branch(cond, &then_block, &cont_block);

    // build then block
    self.builder.position_at_end(&then_block);
    self.compile_node(then_body, func, funcs);
    self.builder.build_unconditional_branch(&cont_block);

    // let then_bb = self.builder.get_insert_block().unwrap();

    self.builder.build_unconditional_branch(&cont_block);

    // emit merge block
    self.builder.position_at_end(&cont_block);

    // let phi = self.builder.build_phi(self.context.i32_type(), "iftmp");
  }

  fn compile_block(
    &mut self,
    body_start: &Node,
    name: &str,
    func: &FunctionValue,
    funcs: &HashMap<String, Func>,
  ) -> BasicBlock {
    let mut next_node = Some(body_start);
    let block = self.context.append_basic_block(func, name);
    self.builder.position_at_end(&block);
    while match next_node {
      Some(_) => true,
      _ => false,
    } {
      self.compile_node(&next_node.clone().unwrap(), func, funcs);
      next_node = next_node.unwrap().get_next_instruction();
    }
    // self.builder.position_at_end(&block);

    println!("Returning");
    return block;
  }
}
