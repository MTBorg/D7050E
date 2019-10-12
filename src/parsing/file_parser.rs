use crate::{errors::parse_error::ParseError, func::Func};
use std::collections::HashMap;

fn get_error_line_from_byte_offset(
  file: &str,
  error_index: usize,
) -> (usize, String, usize) {
  if error_index >= file.len() {
    panic!(
      "Received index ({}) larger than file size ({})",
      error_index,
      file.len()
    );
  }
  let mut n = error_index;
  let mut line_number: usize = 0;
  let mut error_line: String = String::from("");
  let mut error_offset: usize = 0;
  for line in file.split("\n") {
    if n < line.len() + 1 {
      error_line = line.to_string();
      error_offset = n;
      break;
    }
    n -= line.len() + 1;
    line_number += 1;
  }
  return (line_number, error_line, error_offset);
}

pub fn parse(file: String) -> Result<HashMap<String, Func>, ParseError> {
  let res = crate::parsing::grammar::FileParser::new().parse(file.as_str());
  return match res {
    Ok(s) => Ok(s),
    Err(e) => match e {
      lalrpop_util::ParseError::InvalidToken { location } => {
        let (err_line_num, err_string, err_offset) =
          get_error_line_from_byte_offset(&file, location);
        return Err(ParseError::InvalidToken{location: err_offset,
                line: err_string,
                line_num: err_line_num,
                });
      }
      lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
        let (start, token, end) = token;
        let (line_num, err_string, err_offset1) = get_error_line_from_byte_offset(&file, start);
        let (_, _, err_offset2) = get_error_line_from_byte_offset(&file, end);
        return Err(ParseError::UnrecognizedToken {
          start: err_offset1,
          end: err_offset2,
          line: err_string,
          line_num: line_num,
          token: token.1.to_string(),
          expected_tokens: expected,
        });
      }
      _ => unimplemented!("Unsupported lalrpop error message"),
    },
  };
}

#[cfg(test)]
mod tests {
  use super::parse;
  use crate::parsing::expr_parser::Opcode;

  #[test]
  pub fn test_parse_fibbonacci_recursive() {
    assert!(parse(
      "
            fn fib_rec(n: i32) -> i32{
                return fib_rec(n - 1);
            }
            
            fn main(){
                return fib_rec(9);
            }"
      .to_string()
    )
    .is_ok())
  }
}
