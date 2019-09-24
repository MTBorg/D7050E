use super::ParseError;
use crate::node::Node;

pub fn parse(s: &str) -> Result<Box<Node>, ParseError> {
  let res = crate::parsing::grammar::IfParser::new().parse(s);
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
  pub fn test_if_parser_if_var() {
    assert!(parse(
      "
        if a {
            let a = 2;
        }
        "
    )
    .is_ok());
  }

  #[test]
  pub fn test_if_parser_empty_body() {
    assert!(parse(
      "
        if a {
        }
        "
    )
    .is_ok());
  }

  #[test]
  pub fn test_if_parser_no_condition() {
    assert!(!parse(
      "
        if {
            let a = 1;
        }
        "
    )
    .is_ok());
  }

  #[test]
  pub fn test_if_parser_nested_ifs() {
    assert!(parse(
      "
        if a {
            let d = 2;
            if b {
                let c = 1;
            }
            let e = 3;
        }
        "
    )
    .is_ok());
  }

  #[test]
  pub fn test_if_parser_if_else() {
    assert!(parse(
      "
        if a {
            let a = 2;
        } else {
            let b = 3;
        }
        "
    )
    .is_ok());
  }

  #[test]
  pub fn test_if_parser_if_else_nested() {
    assert!(parse(
      "
        if a {
            let a = 2;
            if b {
                let c = 3;
            } else{
                let a = 4;
            }
        } else {
            let b = 3;
            if d{
                let a = 4;
            }
        }
        "
    )
    .is_ok());
  }

  #[test]
  pub fn test_if_parser_bool_expr_eq() {
    assert!(parse(
      "
        if a == 2 {
            let a = 2;
        }
        "
    )
    .is_ok());
  }

  #[test]
  pub fn test_if_parser_bool_expr_neq() {
    assert!(parse(
      "
        if a != 2 {
            let a = 2;
        }
        "
    )
    .is_ok());
  }

  #[test]
  pub fn test_if_parser_bool_expr_and() {
    assert!(parse(
      "
        if a && b {
            let a = 2;
        }
        "
    )
    .is_ok());
  }

  #[test]
  pub fn test_if_parser_bool_expr_or() {
    assert!(parse(
      "
        if a || b {
            let a = 2;
        }
        "
    )
    .is_ok());
  }

  #[test]
  pub fn test_if_parser_bool_expr_else() {
    assert!(parse(
      "
        if a == 2 {
            let a = 2;
        }else{
            let a = 3;
        }
        "
    )
    .is_ok());
  }

  #[test]
  pub fn test_if_parser_bool_expr_mult() {
    assert!(parse(
      "
        if a * 5 == 2 {
            let a = 2;
        }"
    )
    .is_ok());
  }
}
