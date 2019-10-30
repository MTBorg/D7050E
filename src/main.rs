#[macro_use]
extern crate lalrpop_util;

#[macro_use]
mod util;
mod errors;
mod interpreter;
mod llvm;
mod parsing;
mod type_checker;
mod types;

use std::{convert::TryFrom, path::Path};

use llvm::Compiler;
use type_checker::type_check_program;
use types::program::Program;

fn print_error_header() {
  println!("Errors");
  println!("==============================");
}

fn main() {
  let program = match Program::try_from(Path::new("input.rs")) {
    Ok(program) => program,
    Err(e) => {
      print_error_header();
      println!("{}", e);
      return;
    }
  };
  // let type_res = type_check_program(&program);
  // if let Ok(_) = type_res {
  //   println!(
  //     "Interpreter finished with exit code {}",
  //     match program.run() {
  //       Some(value) => value.to_string(),
  //       None => 0.to_string(),
  //     }
  //   )
  // } else if let Err(errors) = type_res {
  //   print_error_header();
  //   for error in errors.iter() {
  //     println!("- {}", error);
  //   }
  // }
  let mut compiler = Compiler::new();

  let main = compiler
    .compile_program(&program)
    .ok_or("Unable to JIT compile program")
    .unwrap();

  unsafe {
    println!("Program exited with exit code {}", main.call());
  }
}
