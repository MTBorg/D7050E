use super::ParseError;
use crate::node::Node;

pub fn parse(s: &str) -> Result<Box<Node>, ParseError>{
    let res = crate::parsing::grammar::BodyParser::new().parse(s);
    return match res{
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError{message: e.to_string()}),
    }
}

#[cfg(test)]
mod tests{
    use super::parse;

    #[test]
    pub fn test_body_parser_one_instruction(){
        assert!(parse("
            {
                let a = 2;
            }
        ").is_ok());
    }

    #[test]
    pub fn test_body_parser_two_instructions(){
        assert!(parse("
            {
                let a = 2;
                let b = 3;
            }
        ").is_ok());
    }

    #[test]
    pub fn test_body_parser_three_instructions(){
        assert!(parse("
            {
                let a = 3 + 1;
                let b = a - 2;
                let c = 1000 -240;
            }
        ").is_ok());
    }

    #[test]
    pub fn test_body_parser_multiple_statements_one_line(){
        assert!(parse("
            {
                let a = 3 + 1; let b = a - 2; let c = 1000 -240;
            }
        ").is_ok());
    }

    #[test]
    pub fn test_body_parser_missing_start_bracket(){
        assert!(!parse("
                let a = 3 + 1;
                let c = 1000 -240;
            }
        ").is_ok());
    }

    #[test]
    pub fn test_body_parser_missing_trailing_bracket(){
        assert!(!parse("
            {
                let a = 3 + 1;
                let b = a - 2;
                let c = 1000 -240;
        ").is_ok());
    }

    #[test]
    pub fn test_body_parser_missing_semicolon_first_line(){
        assert!(!parse("
            {
                let a = 3 + 1
                let b = a - 2;
                let c = 1000 -240;
        ").is_ok());
    }

    #[test]
    pub fn test_body_parser_missing_semicolon_middle_line(){
        assert!(!parse("
            {
                let a = 3 + 1;
                let b = a - 2
                let c = 1000 -240;
        ").is_ok());
    }

    #[test]
    pub fn test_body_parser_missing_semicolon_last_line(){
        assert!(!parse("
            {
                let a = 3 + 1;
                let b = a - 2;
                let c = 1000 -240
        ").is_ok());
    }

    #[test]
    pub fn test_body_parser_empty_body(){
        assert!(parse("
        {

        }").is_ok());
    }
}
