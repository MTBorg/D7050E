use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use std::collections::HashMap;

use crate::interpreter::eval;
use crate::types::{
  context::Context as VarContext, func::Func, node::Node, program::Program,
  variable::Variable,
};
use inkwell::values::IntValue;

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
  pub variables: HashMap<String, Variable>,
}

impl Compiler {
  pub fn new() -> Self {
    let context = Context::create();
    Compiler {
      builder: context.create_builder(),
      module: context.create_module("main"),
      context: context,
      variables: HashMap::new(),
    }
  }

  fn compile_expr(
    &self,
    expr: &Node,
    context: &Context,
    mut var_context: &mut VarContext<Variable>,
    funcs: &HashMap<String, Func>,
  ) -> IntValue {
    let val = eval(expr, &mut var_context, funcs);
    return match val {
      Node::Number(n) => context.i32_type().const_int(n as u64, false),
      _ => unreachable!(),
    };
  }

  pub fn compile_program(&self, program: &Program) -> Option<JitFunction<MainFunc>> {
    let execution_engine = self
      .module
      .create_jit_execution_engine(OptimizationLevel::None)
      .unwrap();
    let i32_type = self.context.i64_type();
    let fn_type = i32_type.fn_type(&[], false);

    for (_, func) in program.funcs.iter() {
      let function = self.module.add_function(&func.name, fn_type, None);
      let basic_block = self.context.append_basic_block(&function, "entry");
      self.builder.position_at_end(&basic_block);
      self.compile_func(func, &program.funcs);
    }

    self
      .builder
      .build_return(Some(&i32_type.const_int(0, false)));

    unsafe { execution_engine.get_function("main").ok() }
  }

  fn compile_func(&self, func: &Func, funcs: &HashMap<String, Func>) {
    self.compile_node(&func.body_start, &mut VarContext::from(func), funcs);
  }

  fn compile_node(
    &self,
    node: &Node,
    var_context: &mut VarContext<Variable>,
    funcs: &HashMap<String, Func>,
  ) {
    match node {
      Node::Return(expr, _) => {
        let expr_val = self.compile_expr(expr, &self.context, var_context, funcs);
        self.builder.build_return(Some(&expr_val));
      }
      // Node::Let(id, _, _, expr, _) => {
      //   let expr_val = self.compile_expr(expr, &self.context, var_context, funcs);
      // }
      _ => unreachable!(),
    };
  }
}
