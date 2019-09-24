use super::ParseError;
use crate::func::Func;
use crate::node::Node;
use std::collections::HashMap;

pub fn parse(file: String) -> Result<HashMap<String, Func>, ParseError> {
  let res = crate::parsing::grammar::FileParser::new().parse(file.as_str());
  return match res {
    Ok(s) => Ok(s),
    Err(e) => Err(ParseError {
      message: e.to_string(),
    }),
  };
}
