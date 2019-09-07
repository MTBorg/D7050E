lalrpop_mod!(pub expr);

#[derive(Debug)]
pub struct ParseError{
    message: String
}

pub mod expr_parser{
    use crate::parsing::ParseError;

    #[derive(Debug)]
    pub enum Expr{
        Number(i32),
        Var(String),
        Op(Box<Expr>, Opcode, Box<Expr>)
    }
    
    #[derive(Debug)]
    pub enum Opcode{
        Mul, Div, Add, Sub
    }

    pub fn parse(s: &str) -> Result<Box<Expr>, ParseError>{
        let  res = crate::parsing::expr::ExprParser::new().parse(s);
        return match res{
            Ok(s) => Ok(s),
            Err(e) => Err(ParseError{message: e.to_string()}),
        }
    }
}

#[cfg(test)]
mod tests{
    #[cfg(test)]
    mod expr{
        use crate::parsing::expr::*;

        #[test]
        fn test_parse_number_no_parens() {
            assert!(ExprParser::new().parse("123").is_ok());
        }

        #[test]
        fn test_parse_number_parens() {
            assert!(ExprParser::new().parse("(458)").is_ok());
        }

        #[test]
        fn test_parse_var_no_parens() {
            assert!(ExprParser::new().parse("abc").is_ok());
        }

        #[test]
        fn test_parse_var_parens() {
            assert!(ExprParser::new().parse("(abc)").is_ok());
        }

        #[test]
        fn test_parse_var_trailing_number() {
            assert!(ExprParser::new().parse("x3").is_ok());
        }

        #[test]
        fn test_parse_var_no_starting_number() {
            assert!(!ExprParser::new().parse("(234var)").is_ok());
        }

        #[test]
        fn test_parse_var_no_space() {
            assert!(!ExprParser::new().parse("var1 var2").is_ok());
        }

        #[test]
        fn test_parse_correct_expressions() {
            assert!(ExprParser::new().parse("1+2+3").is_ok());
            assert!(ExprParser::new().parse("1 * 3 + 2 / 1").is_ok());
            assert!(ExprParser::new().parse("1 +    2   ").is_ok());
            assert!(ExprParser::new().parse("a + b").is_ok()); // Var + var
            assert!(ExprParser::new().parse("a - b").is_ok()); // Var - var
            assert!(ExprParser::new().parse("a * b").is_ok()); // Var * var
            assert!(ExprParser::new().parse("a / b").is_ok()); // Var / var
            assert!(ExprParser::new().parse("1 + a").is_ok()); // Literal + var
            assert!(ExprParser::new().parse("2 - a").is_ok()); // Literal - var
            assert!(ExprParser::new().parse("5 * a").is_ok()); // Literal * var
            assert!(ExprParser::new().parse("7 / a").is_ok()); // Literal / var
            assert!(ExprParser::new().parse("a + 1").is_ok()); // Var + Literal
            assert!(ExprParser::new().parse("a - 2").is_ok()); // Var - Literal
            assert!(ExprParser::new().parse("a * 3").is_ok()); // Var * Literal
            assert!(ExprParser::new().parse("a / 4").is_ok()); // Var / Literal
        }

        #[test]
        fn test_parse_incorrect_expressions() {
            assert!(!ExprParser::new().parse("+1 - 2").is_ok()); // Prefixed operator
            assert!(!ExprParser::new().parse("1++2").is_ok()); // Double operator
            assert!(!ExprParser::new().parse("1 + ! 2").is_ok()); // Unknown operator
        }
    }
}
