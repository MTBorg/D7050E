use super::ParseError;
use crate::node::Node;

pub fn parse(s: &str) -> Result<Box<Node>, ParseError>{
    let res = crate::parsing::grammar::BodyParser::new().parse(s);
    return match res{
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError{message: e.to_string()}),
    }
}
