#[macro_use]
extern crate lalrpop_util;

#[macro_use]
mod util;
mod errors;
mod interpreter;
mod parsing;
mod type_checker;
mod types;

use std::{convert::TryFrom, path::Path};

use type_checker::type_check_program;
use types::{func::Func, node::Node, program::Program};

// fn main() {
//   let program = match Program::try_from(Path::new("input.rs")) {
//     Ok(program) => program,
//     Err(e) => {
//       print_error_header();
//       println!("{}", e);
//       return;
//     }
//   };
//   let type_res = type_check_program(&program);
//   if let Ok(_) = type_res {
//     println!(
//       "Interpreter finished with exit code {}",
//       match program.run() {
//         Some(value) => value.to_string(),
//         None => 0.to_string(),
//       }
//     )
//   } else if let Err(errors) = type_res {
//     print_error_header();
//     for error in errors.iter() {
//       println!("- {}", error);
//     }
//   }
// }

fn print_error_header() {
  println!("Errors");
  println!("==============================");
}

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::OptimizationLevel;
use std::error::Error;

/// Convenience type alias for the `sum` function.
///
/// Calling this is innately `unsafe` because there's no guarantee it doesn't
/// do `unsafe` operations internally.
type MainFunc = unsafe extern "C" fn() -> i32;

fn main() {
  let program = match Program::try_from(Path::new("input.rs")) {
    Ok(program) => program,
    Err(e) => {
      print_error_header();
      println!("{}", e);
      return;
    }
  };
  let type_res = type_check_program(&program);
  if let Ok(_) = type_res {
    println!(
      "Interpreter finished with exit code {}",
      match program.run() {
        Some(value) => value.to_string(),
        None => 0.to_string(),
      }
    )
  } else if let Err(errors) = type_res {
    print_error_header();
    for error in errors.iter() {
      println!("- {}", error);
    }
  }

  let context = Context::create();
  let module = context.create_module("main");
  let builder = context.create_builder();
  let execution_engine = module
    .create_jit_execution_engine(OptimizationLevel::None)
    .unwrap();

  let main =
    jit_compile_program(&context, &module, &builder, &execution_engine, &program)
      .ok_or("Unable to JIT compile program")
      .unwrap();

  unsafe {
    println!("Program exited with exit code {}", main.call());
  }
}

fn jit_compile_node(
  context: &Context,
  module: &Module,
  builder: &Builder,
  execution_engine: &ExecutionEngine,
  node: &Node,
) {
  match node {
    Node::Return(_, _) => {
      builder.build_return(Some(&context.i32_type().const_int(4, false)));
    }
    _ => unreachable!(),
  }
}

fn jit_compile_func(
  context: &Context,
  module: &Module,
  builder: &Builder,
  execution_engine: &ExecutionEngine,
  func: &Func,
) {
  jit_compile_node(context, module, builder, execution_engine, &func.body_start);
}
fn jit_compile_program(
  context: &Context,
  module: &Module,
  builder: &Builder,
  execution_engine: &ExecutionEngine,
  program: &Program,
) -> Option<JitFunction<MainFunc>> {
  let i32_type = context.i64_type();
  let fn_type = i32_type.fn_type(&[], false);

  for (_, func) in program.funcs.iter() {
    let function = module.add_function(&func.name, fn_type, None);
    let basic_block = context.append_basic_block(&function, "entry");
    builder.position_at_end(&basic_block);
    jit_compile_func(context, module, builder, execution_engine, func);
  }

  // let sum = builder.build_int_add(
  //   context.i32_type().const_int(1, false),
  //   context.i32_type().const_int(1, false),
  //   "sum",
  // );

  // builder.build_return(Some(&sum));
  //

  builder.build_return(Some(&i32_type.const_int(0, false)));

  unsafe { execution_engine.get_function("main").ok() }
}
