#[macro_use] extern crate lalrpop_util;

#[macro_use] mod util;
mod parsing;

use parsing::{expr_parser};

fn main(){
    debug_print!(expr_parser::parse("123abc"));
}
