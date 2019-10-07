use super::ParseError;
use crate::func::Func;

#[allow(dead_code)]
pub fn parse(s: &str) -> Result<Func, ParseError> {
  let res = crate::parsing::grammar::FuncDecParser::new().parse(s);
  return match res {
    Ok(s) => Ok(s),
    Err(e) => Err(ParseError {
      message: e.to_string(),
    }),
  };
}
