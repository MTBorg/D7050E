#[macro_use] extern crate lalrpop_util;

#[macro_use] mod util;
mod node;
mod parsing;
mod func;
mod func_param;
mod interpreter;
mod types;
mod scope;

use std::collections::HashMap;

// TODO: Remove this eventually
#[allow(unused_imports)]
use parsing::{
    expr_parser,
    func_call_parser,
    func_dec_parser,
    let_parser,
    body_parser,
    if_parser,
    file_parser,
};

use func::FuncDec;

fn run_program(funcs: &HashMap<String, FuncDec>){
    match funcs.get("main") {
        Some(main) => main.execute(&vec!(), funcs),
        None => panic!("No main function found")
    }
}

fn main(){
     let input = "
         fn foo(a: i32){
             $DEBUG_CONTEXT
         }

         fn main(){
             foo(a);
         }
     ";
    let funcs = file_parser::parse(input).unwrap();
    run_program(&funcs);
}
