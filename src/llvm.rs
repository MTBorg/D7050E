use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use std::collections::HashMap;

use crate::interpreter::eval;
use crate::types::{
  context::Context as VarContext, func::Func, node::Node, program::Program, value::Value,
  variable::Variable,
};
use inkwell::values::{FunctionValue, IntValue, PointerValue};

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

  fn compile_expr(
    &mut self,
    expr: &Node,
    mut var_context: &mut VarContext<Variable>,
    funcs: &HashMap<String, Func>,
  ) -> IntValue {
    // let val = eval(expr, &mut var_context, funcs);
    match expr {
      Node::Number(n) => return self.context.i32_type().const_int(*n as u64, false),
      Node::Var(name) => {
        let var = self.get_variable(&name);
        return self.builder.build_load(*var, &name).into_int_value();
      }
      Node::Op(left, op, right) => {
        let left_val = self.compile_expr(left, var_context, funcs);
        let right_val = self.compile_expr(right, var_context, funcs);
        return left_val.const_add(right_val);
      }
      _ => unreachable!(),
    };
    // return match val {
    //   Node::Number(n) => self.context.i32_type().const_int(n as u64, false),
    //   Node::Var(name) => {
    //     let var = self.get_variable(&name);
    //     return self.builder.build_load(*var, &name).into_int_value();
    //   }
    //   _ => unreachable!(),
    // };
  }

  pub fn compile_program(&mut self, program: &Program) -> Option<JitFunction<MainFunc>> {
    let execution_engine = self
      .module
      .create_jit_execution_engine(OptimizationLevel::None)
      .unwrap();
    let i32_type = self.context.i64_type();
    let fn_type = i32_type.fn_type(&[], false);

    for (_, func) in program.funcs.iter() {
      let function = self.module.add_function(&func.name, fn_type, None);
      let basic_block = self.context.append_basic_block(&function, "entry");
      self.funcs.insert(func.name.to_string(), function);
      self.builder.position_at_end(&basic_block);
      self.compile_func(func, &program.funcs);
    }

    self
      .builder
      .build_return(Some(&i32_type.const_int(0, false)));

    unsafe { execution_engine.get_function("main").ok() }
  }

  fn compile_func(&mut self, func: &Func, funcs: &HashMap<String, Func>) {
    let mut next_node = Some(&func.body_start);
    let mut context = VarContext::from(func);
    while match next_node {
      Some(_) => true,
      _ => false,
    } {
      self.compile_node(&next_node.unwrap(), &mut context, funcs);
      next_node = next_node.unwrap().get_next_instruction();
    }
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
    var_context: &mut VarContext<Variable>,
    funcs: &HashMap<String, Func>,
  ) {
    match node {
      Node::Return(expr, _) => {
        let expr_val = self.compile_expr(expr, var_context, funcs);
        self.builder.build_return(Some(&expr_val));
      }
      Node::Let(id, _, _, expr, _) => {
        let expr_val = self.compile_expr(expr, var_context, funcs);
        let alloca = self.create_entry_block_alloca("main", id);
        self.builder.build_store(alloca, expr_val);
      }
      _ => unreachable!(),
    };
  }
}
