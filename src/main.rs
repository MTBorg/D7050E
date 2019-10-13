#[macro_use]
extern crate lalrpop_util;

#[macro_use]
mod util;
mod context;
mod errors;
mod func;
mod func_param;
mod interpreter;
mod node;
mod opcode;
mod parsing;
mod program;
mod scope;
mod type_checker;
mod types;
mod value;
mod variable;

use std::{convert::TryFrom, path::Path};

// TODO: Remove this eventually
#[allow(unused_imports)]
use parsing::file_parser;
use program::Program;
use type_checker::type_check_program;

fn main() {
  let program = match Program::try_from(Path::new("input.rs")) {
    Ok(program) => program,
    Err(e) => {
      println!("Errors");
      println!("==============================");
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
    println!("ERRORS");
    println!("=================================");
    for error in errors.iter() {
      println!("- {}", error);
    }
  }
}
