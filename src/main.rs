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
mod value;
mod variable;
mod types;

use std::{collections::HashMap, fs::File, path::Path, io::prelude::*};

// TODO: Remove this eventually
use context::Context;
use func::FuncDec;
#[allow(unused_imports)]
use parsing::{
  body_parser, expr_parser, file_parser, func_call_parser, func_dec_parser, if_parser,
  let_parser,
};

fn run_program(funcs: &HashMap<String, FuncDec>) {
  let mut context = Context::new();
  match funcs.get("main") {
    Some(main) => main.execute(&vec![], funcs, &mut context),
    None => panic!("No main function found"),
  }
}

fn main() {
  let mut file = match File::open(Path::new("input.rs")) {
    Err(e) => panic!("Could not open input file: {}", e),
    Ok(file) => file,
  };

  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(e) => panic!("Could not read input file: {}", e),
    Ok(_) => (),
  }

  let funcs = file_parser::parse(s).unwrap();
  run_program(&funcs);
}
