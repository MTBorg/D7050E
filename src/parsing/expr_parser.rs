use super::ParseError;
use crate::node::Node;


#[derive(Debug)]
pub enum Opcode{
    Mul, Div, Add, Sub
}

pub fn parse(s: &str) -> Result<Box<Node>, ParseError>{
    let  res = crate::parsing::grammar::ExprParser::new().parse(s);
    return match res{
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError{message: e.to_string()}),
    }
}

#[cfg(test)]
mod tests{
    use super::parse;

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
}
