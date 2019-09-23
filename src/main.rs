#[macro_use] extern crate lalrpop_util;

#[macro_use] mod util;
mod node;
mod parsing;
mod func;
mod func_param;
mod interpreter;
mod context;
mod scope;
mod value;
mod variable;

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
use context::Context;

fn run_program(funcs: &HashMap<String, FuncDec>){
    let mut context = Context::new();
    match funcs.get("main") {
        Some(main) => main.execute(&vec!(), funcs, &mut context),
        None => panic!("No main function found")
    }
}

fn main(){
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
