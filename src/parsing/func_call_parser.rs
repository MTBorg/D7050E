use super::ParseError;

pub fn parse(s: &str) -> Result<usize, ParseError>{
    let res = crate::parsing::grammar::FuncCallParser::new().parse(s);
    return match res{
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError{message: e.to_string()}),
    }
}
