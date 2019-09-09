lalrpop_mod!(pub grammar);

pub mod expr_parser;
pub mod func_call_parser;
pub mod func_dec_parser;
pub mod let_parser;

#[derive(Debug)]
pub struct ParseError{
    message: String
}