use super::ParseError;
use crate::node::Node;

#[derive(Debug, PartialEq, Clone)]
pub enum Opcode {
  Mul,
  Div,
  Add,
  Sub,
  Eq,
  Neq,
  And,
  Or,
  Geq,
  Leq,
  Gneq,
  Lneq,
}

impl Opcode {
  pub fn to_str(&self) -> &'static str {
    match self {
      Opcode::Mul => "*",
      Opcode::Div => "/",
      Opcode::Add => "+",
      Opcode::Sub => "-",
      Opcode::Eq => "==",
      Opcode::Neq => "!=",
      Opcode::And => "&&",
      Opcode::Or => "||",
      Opcode::Geq => ">=",
      Opcode::Leq => "<=",
      Opcode::Gneq => ">",
      Opcode::Lneq => "<",
    }
  }
}

#[allow(dead_code)]
pub fn parse(s: &str) -> Result<Box<Node>, ParseError> {
  let res = crate::parsing::grammar::ExprParser::new().parse(s);
  return match res {
    Ok(s) => Ok(s),
    Err(e) => Err(ParseError {
      message: e.to_string(),
    }),
  };
}

#[cfg(test)]
mod tests {
  use super::{parse, Node, Opcode};

  #[test]
  fn test_parse_number_no_parens() {
    assert!(parse("123").is_ok());
  }

  #[test]
  fn test_parse_number_parens() {
    assert!(parse("(458)").is_ok());
  }

  #[test]
  fn test_parse_var_no_parens() {
    assert!(parse("abc").is_ok());
  }

  #[test]
  fn test_parse_var_parens() {
    assert!(parse("(abc)").is_ok());
  }

  #[test]
  fn test_parse_var_trailing_number() {
    assert!(parse("x3").is_ok());
  }

  #[test]
  fn test_parse_var_no_starting_number() {
    assert!(!parse("(234var)").is_ok());
  }

  #[test]
  fn test_parse_var_no_space() {
    assert!(!parse("var1 var2").is_ok());
  }

  #[test]
  fn test_parse_parenthesized_addition() {
    assert!(parse("(1 + 4)").is_ok());
  }

  #[test]
  fn test_parse_nested_parenthesis() {
    assert!(parse("(1 + (4 * 5))").is_ok());
  }

  #[test]
  fn test_parse_addition_var_int() {
    assert!(parse("n + 2").is_ok());
  }

  #[test]
  fn test_parse_correct_expressions() {
    assert!(parse("1+2+3").is_ok());
    assert!(parse("1 * 3 + 2 / 1").is_ok());
    assert!(parse("1 +    2   ").is_ok());
    assert!(parse("a + b").is_ok()); // Var + var
    assert!(parse("a - b").is_ok()); // Var - var
    assert!(parse("a * b").is_ok()); // Var * var
    assert!(parse("a / b").is_ok()); // Var / var
    assert!(parse("1 + a").is_ok()); // Literal + var
    assert!(parse("2 - a").is_ok()); // Literal - var
    assert!(parse("5 * a").is_ok()); // Literal * var
    assert!(parse("7 / a").is_ok()); // Literal / var
    assert!(parse("a + 1").is_ok()); // Var + Literal
    assert!(parse("a - 2").is_ok()); // Var - Literal
    assert!(parse("a * 3").is_ok()); // Var * Literal
    assert!(parse("a / 4").is_ok()); // Var / Literal
  }

  #[test]
  fn test_parse_incorrect_expressions() {
    assert!(!parse("+1 - 2").is_ok()); // Prefixed operator
    assert!(!parse("1++2").is_ok()); // Double operator
    assert!(!parse("1 + ! 2").is_ok()); // Unknown operator
  }

  #[test]
  fn test_precedence_1() {
    assert_eq!(
      parse("1+2*3").unwrap(),
      Box::new(Node::Op(
        Box::new(Node::Number(1)),
        Opcode::Add,
        Box::new(Node::Op(
          Box::new(Node::Number(2)),
          Opcode::Mul,
          Box::new(Node::Number(3))
        ))
      ))
    )
  }

  #[test]
  fn test_precedence_2() {
    assert_eq!(
      parse("(1+2)*3").unwrap(),
      Box::new(Node::Op(
        Box::new(Node::Op(
          Box::new(Node::Number(1)),
          Opcode::Add,
          Box::new(Node::Number(2))
        )),
        Opcode::Mul,
        Box::new(Node::Number(3))
      ))
    )
  }

  #[test]
  fn test_bool_precedence_1() {
    assert_eq!(
      parse("a && b == c").unwrap(),
      Box::new(Node::Op(
        Box::new(Node::Var("a".to_string())),
        Opcode::And,
        Box::new(Node::Op(
          Box::new(Node::Var("b".to_string())),
          Opcode::Eq,
          Box::new(Node::Var("c".to_string()))
        ))
      ))
    )
  }

  #[test]
  fn test_bool_precedence_2() {
    assert_eq!(
      parse("(a && b) == c").unwrap(),
      Box::new(Node::Op(
        Box::new(Node::Op(
          Box::new(Node::Var("a".to_string())),
          Opcode::And,
          Box::new(Node::Var("b".to_string()))
        )),
        Opcode::Eq,
        Box::new(Node::Var("c".to_string()))
      ))
    )
  }
}
