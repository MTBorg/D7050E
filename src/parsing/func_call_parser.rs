use super::ParseError;
use crate::parsing::grammar::FuncCallParser;

pub fn parse(s: &str) -> Result<usize, ParseError>{
    let res = FuncCallParser::new().parse(s);
    return match res{
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError{message: e.to_string()}),
    }
}

#[cfg(test)]
mod tests{
    use super::FuncCallParser;

    #[test]
    pub fn test_no_arguments(){
        assert!(FuncCallParser::new().parse("foo()").is_ok());
    }

    #[test]
    pub fn test_no_parenthesis_no_arguments(){
        assert!(!FuncCallParser::new().parse("foo").is_ok());
    }

    #[test]
    pub fn test_no_parenthesis_with_arguments(){
        assert!(!FuncCallParser::new().parse("foo 123, abc ").is_ok());
    }

    #[test]
    pub fn test_arguments_single_var(){
        assert!(FuncCallParser::new().parse("foo(bar)").is_ok());
    }

    #[test]
    pub fn test_arguments_single_numeric(){
        assert!(FuncCallParser::new().parse("foo(123)").is_ok());
    }

    #[test]
    pub fn test_arguments_multiple_var(){
        assert!(FuncCallParser::new().parse("foo(bar, var1)").is_ok());
    }

    #[test]
    pub fn test_arguments_multiple_numeric(){
        assert!(FuncCallParser::new().parse("foo(123, 456)").is_ok());
    }

    #[test]
    pub fn test_arguments_numerics_and_vars(){
        assert!(FuncCallParser::new().parse("foo(123, abc, 456, 789, bar)").is_ok());
    }

    #[test]
    pub fn test_missing_argument(){
        assert!(!FuncCallParser::new().parse("foo(123, , bar)").is_ok());
    }

    #[test]
    pub fn test_missing_commas(){
        assert!(!FuncCallParser::new().parse("foo(123 xyz bar)").is_ok());
    }

    #[test]
    pub fn test_trailing_comma(){
        assert!(!FuncCallParser::new().parse("foo(a, b,)").is_ok());
    }
}
