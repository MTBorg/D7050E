#[macro_use] extern crate lalrpop_util;

#[macro_use] mod util;
mod node;
mod parsing;
mod func;
mod funcParam;

use parsing::{
    expr_parser,
    func_call_parser,
    func_dec_parser
};

fn main(){
    // debug_print!(expr_parser::parse("123abc"));
    // debug_print!(func_parser::parse("fn test(hello, world)"));
    // debug_print!(func_parser::parse("fn test()"));
    debug_print!(func_dec_parser::parse("fn test(a: i32) -> i32{}"));
}
