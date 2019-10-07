use super::ParseError;
use crate::node::Node;
use crate::parsing::grammar::FuncCallParser;

#[allow(dead_code)]
pub fn parse(s: &str) -> Result<Box<Node>, ParseError> {
  let res = FuncCallParser::new().parse(s);
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
  pub fn test_no_arguments() {
    assert!(parse("foo()").is_ok());
  }

  #[test]
  pub fn test_no_parenthesis_no_arguments() {
    assert!(!parse("foo").is_ok());
  }

  #[test]
  pub fn test_no_parenthesis_with_arguments() {
    assert!(!parse("foo 123, abc ").is_ok());
  }

  #[test]
  pub fn test_arguments_single_var() {
    assert!(parse("foo(bar)").is_ok());
  }

  #[test]
  pub fn test_arguments_single_numeric() {
    assert!(parse("foo(123)").is_ok());
  }

  #[test]
  pub fn test_arguments_multiple_var() {
    assert!(parse("foo(bar, var1)").is_ok());
  }

  #[test]
  pub fn test_arguments_multiple_numeric() {
    assert!(parse("foo(123, 456)").is_ok());
  }

  #[test]
  pub fn test_arguments_numerics_and_vars() {
    assert!(parse("foo(123, abc, 456, 789, bar)").is_ok());
  }

  #[test]
  pub fn test_missing_argument() {
    assert!(!parse("foo(123, , bar)").is_ok());
  }

  #[test]
  pub fn test_missing_commas() {
    assert!(!parse("foo(123 xyz bar)").is_ok());
  }

  #[test]
  pub fn test_trailing_comma() {
    assert!(parse("foo(a, b,)").is_ok());
  }

  #[test]
  pub fn test_tree_no_args() {
    assert_eq!(
      *parse("foo()").unwrap(),
      Node::FuncCall("foo".to_string(), vec![], None)
    )
  }

  #[test]
  pub fn test_tree_multiple_args() {
    assert_eq!(
      *parse("foo(a, b, c)").unwrap(),
      Node::FuncCall(
        "foo".to_string(),
        vec![
          Node::Var("a".to_string()),
          Node::Var("b".to_string()),
          Node::Var("c".to_string())
        ],
        None
      )
    )
  }

  #[test]
  pub fn test_calc_arg() {
    assert_eq!(
      *parse("foo(2+4)").unwrap(),
      Node::FuncCall(
        "foo".to_string(),
        vec![Node::Op(
          Box::new(Node::Number(2)),
          Opcode::Add,
          Box::new(Node::Number(4))
        )],
        None
      )
    )
  }

  #[test]
  pub fn test_calc_arg_with_var() {
    assert_eq!(
      *parse("foo(n+4)").unwrap(),
      Node::FuncCall(
        "foo".to_string(),
        vec![Node::Op(
          Box::new(Node::Var("n".to_string())),
          Opcode::Add,
          Box::new(Node::Number(4))
        )],
        None
      )
    )
  }
}
