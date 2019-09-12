use super::ParseError;
use crate::node::Node;

#[derive(Debug)]
pub enum BoolOpcode{
    EQ,
    NEQ,
    AND,
    OR
}

pub fn parse(s: &str) -> Result<Box<Node>, ParseError>{
    let res = crate::parsing::grammar::BoolExprParser::new().parse(s);
    return match res{
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError{message: e.to_string()}),
    }
}
