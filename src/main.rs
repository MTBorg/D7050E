#[macro_use] extern crate lalrpop_util;

#[macro_use] mod util;
mod node;
mod parsing;
mod func;
mod func_param;

use parsing::{
    expr_parser,
    func_call_parser,
    func_dec_parser,
    let_parser
};

fn main(){
    // debug_print!(expr_parser::parse("123abc"));
    // debug_print!(func_parser::parse("fn test(hello, world)"));
    // debug_print!(func_parser::parse("fn test()"));
    // debug_print!(func_dec_parser::parse("fn test(a: i32) -> i32{}"));
    let mut n1 = let_parser::parse("let a = b").unwrap();
    let mut n2 = let_parser::parse("let c = a").unwrap();
    let f1 = func_call_parser::parse("foo(a)").unwrap();

    n2.attach_right_most_child(*f1);
    n1.attach_right_most_child(*n2);

    debug_print!(n1);
}
