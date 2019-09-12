use super::ParseError;
use crate::node::Node;

#[derive(Debug)]
pub enum BoolOpcode{
    EQ,
    NEQ,
    AND,
    OR
}

pub fn parse(s: &str) -> Result<Box<Node>, ParseError>{
    let res = crate::parsing::grammar::BoolExprParser::new().parse(s);
    return match res{
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError{message: e.to_string()}),
    }
}

#[cfg(test)]
mod tests{
    use super::parse;

    #[test]
    pub fn test_bool_expr_parser_true(){
        assert!(parse("true").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_true_parenthesized(){
        assert!(parse("( true )").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_false(){
        assert!(parse("false").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_false_parenthesized(){
        assert!(parse("( false )").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_var_eq_num(){
        assert!(parse("a == 2").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_var_neq_num(){
        assert!(parse("a != 2").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_var_or_num(){
        assert!(parse("a || 2").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_var_and_num(){
        assert!(parse("a && 2").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_var_eq_var(){
        assert!(parse("a == b").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_var_neq_var(){
        assert!(parse("a != b").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_var_or_var(){
        assert!(parse("a || b").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_var_and_var(){
        assert!(parse("a && b").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_num_eq_var(){
        assert!(parse("2 == b").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_num_neq_var(){
        assert!(parse("2 != b").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_num_or_var(){
        assert!(parse("2 || b").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_num_and_var(){
        assert!(parse("2 && b").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_num_eq_num(){
        assert!(parse("2 == 2").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_num_neq_num(){
        assert!(parse("2 != 2").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_num_or_num(){
        assert!(parse("2 || 2").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_num_and_num(){
        assert!(parse("2 && 2").is_ok());
    }

    #[test]
    pub fn test_bool_expr_parser_chained_parenthesized(){
        assert!(parse("(a == 2) && b").is_ok());
    }

    // TODO: Remove ignore when not allowing unparenthesized 
    // chained expressions has been fixed
    #[test]
    #[ignore] 
    pub fn test_bool_expr_parser_chained_not_parenthesized(){
        assert!(!parse("a == 2 && b").is_ok());
    }
}
