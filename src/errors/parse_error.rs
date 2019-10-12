#[derive(Debug)]
pub enum ParseError {
  UnrecognizedToken {
    start: usize,
    end: usize,
    line: String,
    line_num: usize,
    token: String,
    expected_tokens: Vec<String>,
  },
  InvalidToken {
    location: usize,
    line: String,
    line_num: usize,
  },
}

const MARKER: &'static str = "^";
const RANGE: &'static str = "~";

/// Prints a marker range on a new line
///
/// # Arguments
///
/// * `start` - The starting position (relative to the beginning of the line) of the
/// start of the marker
/// * `end` - The ending position (relative to the beginning of the line) of the end
/// of the marker
///
/// # Return - a string containing the marker line
fn get_marker_range(start: usize, end: usize) -> String {
  let mut s = match String::from_utf8(vec![b' '; start]) {
    Ok(s) => s,
    Err(e) => panic!(e),
  };
  s += MARKER;
  for _ in start..end - 2 {
    s += RANGE;
  }
  s += MARKER;
  s
}

impl std::fmt::Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      ParseError::UnrecognizedToken {
        start,
        end,
        line,
        line_num,
        token,
        expected_tokens,
      } => {
        let mut s = format!("Unrecognized token {} on line {}: ", token, line_num);
        let marker_start = s.len();
        s += &line;
        let marker_line = get_marker_range(marker_start + start, marker_start + end);

        let mut expectations = String::from("\nExpected:\n");
        for token in expected_tokens.iter() {
          expectations += &((*token).clone() + "\n");
        }

        write!(f, "{}\n{}{}", s, marker_line, expectations)
      }
      ParseError::InvalidToken{location, line, line_num} => {
        let l1 = format!(
          "Err on line {}, character {}: {}",
          line_num, location, line
        );

        //Place the marker
        let mut l2: String = String::from("\n");
        for _ in 0..(location + l1.len() - line.len()) {
          l2 += " ";
        }
        l2 += "^";

        write!(f, "{}", l1 + &l2)
      }
    }
  }
}
