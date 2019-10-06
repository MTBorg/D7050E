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

#[cfg(test)]
mod tests {
  use super::{parse, Node};
  use crate::parsing::expr_parser::Opcode;

  #[test]
  pub fn test_parse_fibbonacci_recursive() {
    assert!(parse(
      "
            fn fib_rec(n: i32) -> i32{
                return fib_rec(n - 1);
            }
            
            fn main(){
                return fib_rec(9);
            }"
      .to_string()
    )
    .is_ok())
  }
}
