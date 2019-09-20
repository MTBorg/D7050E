use super::ParseError;
use crate::node::Node;
use std::collections::HashMap;
use crate::func::FuncDec;

pub fn parse(s: &str) -> Result<HashMap<String, FuncDec>, ParseError>{
    let  res = crate::parsing::grammar::FileParser::new().parse(s);
    return match res{
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError{message: e.to_string()}),
    }
}
