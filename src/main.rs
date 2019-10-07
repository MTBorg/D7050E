#[macro_use]
extern crate lalrpop_util;

#[macro_use]
mod util;
mod context;
mod func;
mod func_param;
mod interpreter;
mod node;
mod parsing;
mod program;
mod scope;
mod type_checker;
mod type_error;
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
  let context = match program.get_main_context() {
    Some(context) => context,
    _ => panic!("No main in program"),
  };
  debug_print!(type_check_program(&program, &context));
  // program.run();
}
