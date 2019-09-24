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

use std::collections::HashMap;

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
  let input = "
         fn foo(b: i32){
             $DEBUG_CONTEXT
         }

         fn main(){
            let a = 5; 
            $DEBUG_CONTEXT
            foo(2);
         }
     ";
  let funcs = file_parser::parse(input).unwrap();
  run_program(&funcs);
}
