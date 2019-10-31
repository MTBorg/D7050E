use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use std::collections::HashMap;

use crate::types::{
  _type::Type, func::Func, node::Node, opcode::Opcode, program::Program,
};
use inkwell::basic_block::BasicBlock;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue, IntValue, PointerValue};
use inkwell::IntPredicate;

/// Convenience type alias for the `sum` function.
///
/// Calling this is innately `unsafe` because there's no guarantee it doesn't
/// do `unsafe` operations internally.
type MainFunc = unsafe extern "C" fn() -> i32;

/// Compiler holds the LLVM state for the compilation
pub struct Compiler {
  context: Context,
  builder: Builder,
  module: Module,

  // Acts as a scope stack, each block will be pushed to the end of the vector.
  // i.e. the latest variables will be at the end of the vector.
  // This enables shadowing.
  variables: Vec<HashMap<String, PointerValue>>,
}

impl Compiler {
  pub fn new() -> Self {
    let context = Context::create();
    Compiler {
      builder: context.create_builder(),
      module: context.create_module("main"),
      context: context,
      variables: vec![],
    }
  }

  fn get_variable(&self, id: &str) -> &PointerValue {
    for block in self.variables.iter().rev() {
      if let Some(var) = block.get(id) {
        return var;
      }
    }
    panic!(
      "Could not find a matching variable, {} in {:?}",
      id, self.variables
    );
  }

  fn compile_expr(&self, expr: &Node, funcs: &HashMap<String, Func>) -> IntValue {
    match expr {
      Node::Number(n) => self.context.i32_type().const_int(*n as u64, false),
      Node::Var(name) => {
        let var = self.get_variable(&name);
        self.builder.build_load(*var, &name).into_int_value()
      }
      Node::Bool(b) => self
        .context
        .i32_type()
        .const_int(if *b { 1 } else { 0 }, false),
      Node::Op(left, op, right) => {
        let left_val = self.compile_expr(left, funcs);
        let right_val = self.compile_expr(right, funcs);
        match op {
          Opcode::Add => self.builder.build_int_add(left_val, right_val, "add"),
          Opcode::Sub => self.builder.build_int_sub(left_val, right_val, "sub"),
          Opcode::Mul => self.builder.build_int_mul(left_val, right_val, "mul"),
          Opcode::Div => self
            .builder
            .build_int_signed_div(left_val, right_val, "div"),
          Opcode::Eq => {
            self
              .builder
              .build_int_compare(IntPredicate::EQ, left_val, right_val, "eq")
          }
          Opcode::Neq => {
            self
              .builder
              .build_int_compare(IntPredicate::NE, left_val, right_val, "neq")
          }
          Opcode::Geq => {
            self
              .builder
              .build_int_compare(IntPredicate::SGE, left_val, right_val, "geq")
          }
          Opcode::Leq => {
            self
              .builder
              .build_int_compare(IntPredicate::SLE, left_val, right_val, "leq")
          }
          Opcode::Gneq => {
            self
              .builder
              .build_int_compare(IntPredicate::SGT, left_val, right_val, "gneq")
          }
          Opcode::Lneq => {
            self
              .builder
              .build_int_compare(IntPredicate::SLT, left_val, right_val, "lneq")
          }
          Opcode::And => self.builder.build_and(left_val, right_val, "and"),
          Opcode::Or => self.builder.build_or(left_val, right_val, "or"),
        }
      }
      Node::FuncCall(func_name, args, _) => {
        let function = self.module.get_function(func_name).unwrap();
        let args: Vec<BasicValueEnum> = args
          .iter()
          .map(|a| self.compile_expr(a, funcs).into())
          .collect();
        let call = self.builder.build_call(function, &args, func_name);
        *call.try_as_basic_value().left().unwrap().as_int_value()
      }
      _ => unreachable!("Cannot compile node {:#?} in expression", expr),
    }
  }

  pub fn compile_program(&mut self, program: &Program) -> Option<JitFunction<MainFunc>> {
    let execution_engine = self
      .module
      .create_jit_execution_engine(OptimizationLevel::None)
      .unwrap();

    // Add all functions to the module before compiling
    for (_, func) in program.funcs.iter() {
      let args: Vec<BasicTypeEnum> = func
        .params
        .iter()
        .map(|param| match param._type {
          Type::Int => self.context.i32_type().into(),
          Type::Bool => self.context.bool_type().into(),
        })
        .collect();
      let fn_type = match func.ret_type {
        Some(ref r#type) => match r#type {
          Type::Int => self.context.i32_type().fn_type(&args, false),
          Type::Bool => self.context.bool_type().fn_type(&args, false),
        },

        None => self.context.void_type().fn_type(&[], false),
      };
      let function = self.module.add_function(&func.name, fn_type, None);
      self.context.append_basic_block(&function, "entry");
    }

    for (_, func) in program.funcs.iter() {
      let function = self.module.get_function(&func.name).unwrap();
      self.compile_func(&function, &func, &func.body_start, &program.funcs);
    }

    let temp = unsafe { execution_engine.get_function("main").ok() };
    // self.module.print_to_stderr();
    return temp;
  }

  fn compile_func(
    &mut self,
    function: &FunctionValue,
    func_dec: &Func,
    func_body_start: &Node,
    funcs: &HashMap<String, Func>,
  ) {
    let basic_block = function.get_first_basic_block().unwrap();
    self.variables.push(HashMap::new()); // Push scope
    for (i, param) in func_dec.params.iter().enumerate() {
      let arg = function.get_nth_param(i as u32).unwrap();

      let alloca = self.create_entry_block_alloca(&basic_block, &param.name);
      self.builder.position_at_end(&basic_block);
      self.builder.build_store(alloca, arg);
    }
    self.compile_block(func_body_start, &basic_block, function, funcs);

    //If the function is of type void we still need to make sure to build a return
    if let None = func_dec.ret_type {
      let default_return_value = self.context.i32_type().const_int(0, false);
      //Main should always return 0
      self.builder.build_return(if func_dec.name == "main" {
        Some(&default_return_value)
      } else {
        None
      });
    }

    self.variables.pop(); // Pop scope
  }

  /// Creates a new stack allocation instruction in the entry block of the function.
  fn create_entry_block_alloca(
    &mut self,
    block: &BasicBlock,
    name: &str,
  ) -> PointerValue {
    let builder = self.context.create_builder();

    match block.get_first_instruction() {
      Some(first_instr) => builder.position_before(&first_instr),
      None => builder.position_at_end(&block),
    }
    let alloca = builder.build_alloca(self.context.i32_type(), name);
    self
      .variables
      .iter_mut()
      .last()
      .unwrap()
      .insert(name.to_string(), alloca);
    alloca
  }

  fn compile_node(
    &mut self,
    node: &Node,
    func: &FunctionValue,
    funcs: &HashMap<String, Func>,
    block: &BasicBlock,
  ) {
    match node {
      Node::Return(expr, _) => {
        let expr_val = self.compile_expr(expr, funcs);
        self.builder.build_return(Some(&expr_val));
      }
      Node::Let(id, _, _, expr, _) => {
        let alloca = self.create_entry_block_alloca(block, id);
        let expr_val = self.compile_expr(expr, funcs);
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
      Node::Assign(variable, expr, _) => {
        let variable = self.get_variable(variable);
        let expr = self.compile_expr(expr, funcs);
        self.builder.build_store(*variable, expr);
      }
      Node::FuncCall(func_name, args, _) => {
        let args: Vec<BasicValueEnum> = args
          .iter()
          .map(|a| self.compile_expr(a, funcs).into())
          .collect();
        let func = self
          .module
          .get_function(func_name)
          .expect(&format!("Could not find function {}", func_name));
        self.builder.build_call(func, &args, func_name);
      }
      Node::Empty => (),
      _ => unreachable!("Cannot compile node {:#?}", node),
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
    self.compile_block(then_body, &then_block, func, funcs);
    self.builder.build_unconditional_branch(&cont_block);

    // build else block
    self.compile_block(else_body, &else_block, func, funcs);
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
    self.compile_block(then_body, &then_block, func, funcs);
    self.builder.build_unconditional_branch(&cont_block);

    self.builder.build_unconditional_branch(&cont_block);

    // emit merge block
    self.builder.position_at_end(&cont_block);

    // let phi = self.builder.build_phi(self.context.i32_type(), "iftmp");
  }

  fn compile_block(
    &mut self,
    body_start: &Node,
    block: &BasicBlock,
    func: &FunctionValue,
    funcs: &HashMap<String, Func>,
  ) {
    self.builder.position_at_end(&block);
    if let Node::Empty = body_start {
      return;
    }
    let mut next_node = Some(body_start);
    self.variables.push(HashMap::new()); // Push scope
    while match next_node {
      Some(_) => true,
      _ => false,
    } {
      self.compile_node(&next_node.clone().unwrap(), func, funcs, block);
      next_node = next_node.unwrap().get_next_instruction();
    }
    self.variables.pop(); // Pop scope
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::program::Program;
  use std::{convert::TryFrom, path::Path};

  #[test]
  fn test_if_statement_true() {
    let program = Program::try_from(Path::new("tests/samples/if_a_eq_2.rs")).unwrap();

    let mut compiler = Compiler::new();
    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 11);
  }

  #[test]
  fn test_if_empty_body() {
    let program = Program::try_from(Path::new("tests/samples/if_empty_body.rs")).unwrap();

    let mut compiler = Compiler::new();
    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 6);
  }

  #[test]
  fn test_if_else_into_else() {
    let program =
      Program::try_from(Path::new("tests/samples/if_else_into_else.rs")).unwrap();

    let mut compiler = Compiler::new();
    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 11);
  }

  #[test]
  fn test_empty_function() {
    let program =
      Program::try_from(Path::new("tests/samples/empty_function.rs")).unwrap();

    let mut compiler = Compiler::new();
    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 0);
  }

  #[test]
  fn test_variable_in_expression_add() {
    let program =
      Program::try_from(Path::new("tests/samples/variable_in_expression_add.rs"))
        .unwrap();

    let mut compiler = Compiler::new();
    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 5);
  }

  #[test]
  fn test_variable_in_expression_sub() {
    let program =
      Program::try_from(Path::new("tests/samples/variable_in_expression_sub.rs"))
        .unwrap();

    let mut compiler = Compiler::new();
    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 5);
  }
  #[test]
  fn test_variable_in_expression_mul() {
    let program =
      Program::try_from(Path::new("tests/samples/variable_in_expression_mul.rs"))
        .unwrap();

    let mut compiler = Compiler::new();
    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 6);
  }
  #[test]
  fn test_variable_in_expression_div() {
    let program =
      Program::try_from(Path::new("tests/samples/variable_in_expression_div.rs"))
        .unwrap();

    let mut compiler = Compiler::new();
    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 2);
  }

  #[test]
  fn test_assign_variable_to_another() {
    let program =
      Program::try_from(Path::new("tests/samples/assign_variable_to_another.rs"))
        .unwrap();

    let mut compiler = Compiler::new();
    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 3);
  }

  #[test]
  fn test_relop_neq_true() {
    let program =
      Program::try_from(Path::new("tests/samples/relop_neq_true.rs")).unwrap();

    let mut compiler = Compiler::new();
    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_ne!(result, 0);
  }

  #[test]
  fn test_relop_neq_false() {
    let program =
      Program::try_from(Path::new("tests/samples/relop_neq_false.rs")).unwrap();

    let mut compiler = Compiler::new();
    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 0);
  }

  #[test]
  fn test_variable_int_assignment() {
    let program =
      Program::try_from(Path::new("tests/samples/variable_int_assignment.rs")).unwrap();

    let mut compiler = Compiler::new();
    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 4);
  }

  #[test]
  fn test_function_call_return_int() {
    let program =
      Program::try_from(Path::new("tests/samples/function_call_return_int.rs")).unwrap();

    let mut compiler = Compiler::new();
    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 5);
  }

  #[test]
  fn test_function_call_return_variable() {
    let program =
      Program::try_from(Path::new("tests/samples/function_call_return_variable.rs"))
        .unwrap();

    let mut compiler = Compiler::new();
    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 7);
  }

  #[test]
  fn test_variable_function_assignment() {
    let program =
      Program::try_from(Path::new("tests/samples/variable_function_assignment.rs"))
        .unwrap();

    let mut compiler = Compiler::new();
    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 24);
  }

  #[test]
  fn test_function_call_return_arg() {
    let program =
      Program::try_from(Path::new("tests/samples/function_call_return_arg.rs")).unwrap();

    let mut compiler = Compiler::new();
    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 17);
  }

  #[test]
  fn test_function_call_return_sum_of_args() {
    let program = Program::try_from(Path::new(
      "tests/samples/function_call_return_sum_of_args.rs",
    ))
    .unwrap();

    let mut compiler = Compiler::new();

    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, -2);
  }

  #[test]
  fn test_function_call_return_local_variable() {
    let program = Program::try_from(Path::new(
      "tests/samples/function_call_return_local_variable.rs",
    ))
    .unwrap();

    let mut compiler = Compiler::new();

    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 9);
  }

  #[test]
  fn test_variable_scope_functions() {
    let program =
      Program::try_from(Path::new("tests/samples/variable_scope_functions.rs")).unwrap();

    let mut compiler = Compiler::new();

    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 3);
  }

  #[test]
  fn test_fibbonacci_recursive() {
    let program =
      Program::try_from(Path::new("tests/samples/fibbonaci_recursive.rs")).unwrap();

    let mut compiler = Compiler::new();

    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 34);
  }

  #[test]
  fn test_shadowing_return_original() {
    let program =
      Program::try_from(Path::new("tests/samples/shadowing_return_original.rs")).unwrap();

    let mut compiler = Compiler::new();

    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 14);
  }

  #[test]
  fn test_shadowing_return_shadowed() {
    let program =
      Program::try_from(Path::new("tests/samples/shadowing_return_shadowed.rs")).unwrap();

    let mut compiler = Compiler::new();

    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 4);
  }

  #[test]
  fn test_variable_redeclare_in_same_scope() {
    let program = Program::try_from(Path::new(
      "tests/samples/variable_redeclare_in_same_scope.rs",
    ))
    .unwrap();

    let mut compiler = Compiler::new();

    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 7);
  }

  #[test]
  fn test_variable_assignment_in_if_block() {
    let program = Program::try_from(Path::new(
      "tests/samples/variable_assignment_in_if_block.rs",
    ))
    .unwrap();

    let mut compiler = Compiler::new();

    let main = compiler.compile_program(&program).unwrap();
    let result;
    unsafe {
      result = main.call();
    };

    assert_eq!(result, 10);
  }

  #[test]
  #[should_panic]
  fn test() {
    let program =
      Program::try_from(Path::new("tests/samples/function_argument_scope.rs")).unwrap();

    let mut compiler = Compiler::new();

    let main = compiler.compile_program(&program).unwrap();
    unsafe {
      main.call();
    };
  }
}
