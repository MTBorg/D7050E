use super::ParseError;
use crate::node::Node;

pub fn parse(s: &str) -> Result<Box<Node>, ParseError>{
    let res = crate::parsing::grammar::LetParser::new().parse(s);
    return match res{
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError{message: e.to_string()}),
    }
}

#[cfg(test)]
mod tests{
    use crate::parsing::grammar::LetParser;

    #[test]
    fn test_parser_let_no_type_int(){
        assert!(LetParser::new().parse("let a = 2").is_ok());
    }

    #[test]
    fn test_parser_let_no_type_float(){
        assert!(LetParser::new().parse("let a = 1.5").is_ok());
    }

    #[test]
    fn test_parser_let_no_type_negative(){
        assert!(LetParser::new().parse("let a = -11").is_ok());
    }

    #[test]
    fn test_parser_let_type_i32(){
        assert!(LetParser::new().parse("let a: i32 = 2").is_ok());
    }

    #[test]
    fn test_parser_let_type_i32_negative(){
        assert!(LetParser::new().parse("let a: i32 = -50").is_ok());
    }

    #[test]
    fn test_parser_let_type_f32(){
        assert!(LetParser::new().parse("let a: f32 = 2.0").is_ok());
    }

    #[test]
    fn test_parser_let_missing_colon(){
        assert!(!LetParser::new().parse("let a: = 7").is_ok());
    }

    #[test]
    fn test_parser_let_missing_value(){
        assert!(!LetParser::new().parse("let a: f32 = ").is_ok());
    }

    #[test]
    fn test_parser_let_missing_comma_and_value(){
        assert!(!LetParser::new().parse("let a f32 = ").is_ok());
    }

    #[test]
    fn test_parser_let_missing_equal_sign(){
        assert!(!LetParser::new().parse("let a: i32 4").is_ok());
    }
}