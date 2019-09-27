use super::ParseError;
use crate::node::Node;

#[allow(dead_code)]
pub fn parse(s: &str) -> Result<Box<Node>, ParseError> {
  let res = crate::parsing::grammar::AssignParser::new().parse(s);
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

  #[test]
  fn test_parse_assign_constant() {
    assert_eq!(
      parse("a = 2;").unwrap(),
      Box::new(Node::Assign(
        "a".to_string(),
        Box::new(Node::Number(2)),
        None
      ))
    );
  }

  #[test]
  fn test_parse_assign_bool_literal() {
    assert_eq!(
      parse("a = true;").unwrap(),
      Box::new(Node::Assign(
        "a".to_string(),
        Box::new(Node::Bool(true)),
        None
      ))
    );
  }

  #[test]
  fn test_parse_assign_var() {
    assert_eq!(
      parse("a = b;").unwrap(),
      Box::new(Node::Assign(
        "a".to_string(),
        Box::new(Node::Var("b".to_string())),
        None
      ))
    );
  }
}
