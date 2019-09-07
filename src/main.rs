#[macro_use] extern crate lalrpop_util;

#[macro_use] mod util;
mod node;
mod parsing;
mod func;

use parsing::{expr_parser, func_parser};

fn main(){
    // debug_print!(expr_parser::parse("123abc"));
    // debug_print!(func_parser::parse("fn test(hello, world)"));
    debug_print!(func_parser::parse("fn test()"));
}
