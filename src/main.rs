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
mod scope;
mod type_error;
mod value;
mod variable;
mod types;
mod program;
mod type_checker;

use std::path::Path;

// TODO: Remove this eventually
#[allow(unused_imports)]
use parsing::{
  body_parser, expr_parser, file_parser, func_call_parser, func_dec_parser, if_parser,
  let_parser,
};
use program::Program;

fn main() {
  Program::from(Path::new("input.rs")).run();
}
