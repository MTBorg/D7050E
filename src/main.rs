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

use interpreter::eval;
use func::FuncDec;

fn run_program(funcs: &HashMap<String, FuncDec>){
    match funcs.get("main") {
        Some(main) => main.execute(&vec!(), funcs),
        None => panic!("No main function found")
    }
}

fn main(){
    // debug_print!(expr_parser::parse("2"));
    // debug_print!(func_parser::parse("fn test(hello, world)"));
    // debug_print!(func_parser::parse("fn test()"));
    // debug_print!(func_dec_parser::parse("fn test(a: i32) -> i32{}"));
    // let mut n1 = let_parser::parse("let a = b * 2;").unwrap();
    // let mut n2 = let_parser::parse("let c = a;").unwrap();
    // let f1 = func_call_parser::parse("foo(a);").unwrap();


    // n2.attach_right_most_child(*f1);
    // n1.attach_right_most_child(*n2);
    //
   // debug_print!(body_parser::parse(" 
    //     {
    //         let a = 2 * b;
    //         let b = 3;
    //     } 
    // "));
    // debug_print!(if_parser::parse("
    // if a + 2 {
    //     let a = 2; 
    //     if b {
    //         let c = 3;
    //     }
    // }"
    // ));
    // debug_print!(n1);
    //
    let a = true;
    let b = true;
    // if 2 == 3 == 4{

    // }
    // debug_print!(bool_expr_parser::parse(""));
    // debug_print!(eval(*expr_parser::parse("5 - 3").unwrap()));
    // debug_print!(eval(*expr_parser::parse("5 + 10").unwrap()));
    // debug_print!(eval(*expr_parser::parse("5 * 10").unwrap()));
    // debug_print!(eval(*expr_parser::parse("5 / 10").unwrap()));
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
    //
    //
    // debug_print!(func_call_parser::parse("
    //     foo();
    // "));

}
