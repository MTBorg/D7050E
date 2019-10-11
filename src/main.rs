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
mod parsing;
mod program;
mod scope;
mod type_checker;
mod types;
mod value;
mod variable;

use std::path::Path;

// TODO: Remove this eventually
#[allow(unused_imports)]
use parsing::{
  body_parser, expr_parser, file_parser, func_call_parser, func_dec_parser, if_parser,
  let_parser,
};
use program::Program;
use type_checker::type_check_program;

fn main() {
  let program = Program::from(Path::new("input.rs"));
  let type_res = type_check_program(&program);
  // println!(
  //   "{}",
  //   match program.run() {
  //     Some(value) => value.to_string(),
  //     None => 0.to_string(),
  //   }
  // );
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
