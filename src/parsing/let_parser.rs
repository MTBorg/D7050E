use super::ParseError;
use crate::node::Node;

#[allow(dead_code)]
pub fn parse(s: &str) -> Result<Box<Node>, ParseError> {
  let res = crate::parsing::grammar::LetParser::new().parse(s);
  return match res {
    Ok(s) => Ok(s),
    Err(e) => Err(ParseError {
      message: e.to_string(),
    }),
  };
}

#[cfg(test)]
mod tests {
  use super::parse;

  #[test]
  fn test_parser_let_no_type_int() {
    assert!(parse("let a = 2;").is_ok());
  }

  #[test]
  fn test_parser_let_no_type_negative() {
    assert!(parse("let a = -11;").is_ok());
  }

  #[test]
  fn test_parser_let_type_i32() {
    assert!(parse("let a: i32 = 2;").is_ok());
  }

  #[test]
  fn test_parser_let_type_i32_negative() {
    assert!(parse("let a: i32 = -50;").is_ok());
  }

  #[test]
  fn test_parser_let_missing_colon() {
    assert!(!parse("let a: = 7;").is_ok());
  }

  #[test]
  fn test_parser_let_missing_equal_sign() {
    assert!(!parse("let a: i32 4;").is_ok());
  }

  #[test]
  fn test_parser_let_missing_semi_colong() {
    assert!(!parse("let a = 2").is_ok());
  }

  #[test]
  fn test_parser_let_assign_variable_notype() {
    assert!(parse("let a = b;").is_ok());
  }

  #[test]
  fn test_parser_let_assign_variable_typed() {
    assert!(parse("let a: i32 = b;").is_ok());
  }
}
