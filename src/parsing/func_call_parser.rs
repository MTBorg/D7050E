use super::ParseError;
use crate::parsing::grammar::FuncCallParser;
use crate::node::Node;

pub fn parse(s: &str) -> Result<Box<Node>, ParseError>{
    let res = FuncCallParser::new().parse(s);
    return match res{
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError{message: e.to_string()}),
    }
}

#[cfg(test)]
mod tests{
    use super::parse;

    #[test]
    pub fn test_no_arguments(){
        assert!(parse("foo()").is_ok());
    }

    #[test]
    pub fn test_no_parenthesis_no_arguments(){
        assert!(!parse("foo").is_ok());
    }

    #[test]
    pub fn test_no_parenthesis_with_arguments(){
        assert!(!parse("foo 123, abc ").is_ok());
    }

    #[test]
    pub fn test_arguments_single_var(){
        assert!(parse("foo(bar)").is_ok());
    }

    #[test]
    pub fn test_arguments_single_numeric(){
        assert!(parse("foo(123)").is_ok());
    }

    #[test]
    pub fn test_arguments_multiple_var(){
        assert!(parse("foo(bar, var1)").is_ok());
    }

    #[test]
    pub fn test_arguments_multiple_numeric(){
        assert!(parse("foo(123, 456)").is_ok());
    }

    #[test]
    pub fn test_arguments_numerics_and_vars(){
        assert!(parse("foo(123, abc, 456, 789, bar)").is_ok());
    }

    #[test]
    pub fn test_missing_argument(){
        assert!(!parse("foo(123, , bar)").is_ok());
    }

    #[test]
    pub fn test_missing_commas(){
        assert!(!parse("foo(123 xyz bar)").is_ok());
    }

    #[test]
    pub fn test_trailing_comma(){
        assert!(parse("foo(a, b,)").is_ok());
    }
}
