use std::{collections::HashMap, fs::File, io::prelude::*, path::Path};

use crate::{
  context::Context, errors::parse_error::ParseError, func::Func,
  parsing::file_parser::parse, value::Value,
};

pub struct Program {
  pub funcs: HashMap<String, Func>,
  file: String,
}

impl std::convert::TryFrom<&Path> for Program {
  type Error = ParseError;
  fn try_from(path: &Path) -> Result<Self, Self::Error> {
    let mut file = match File::open(path) {
      Ok(file) => file,
      Err(e) => panic!("Could not open input file: {}", e),
    };

    let mut s = String::new();
    if let Err(e) = file.read_to_string(&mut s) {
      panic!("Could not read input file: {}", e);
    };

    let mut program = Program {
      funcs: HashMap::new(),
      file: s,
    };
    if let Err(e) = program.parse() {
      return Err(e);
    }
    Ok(program)
  }
}

impl Program {
  pub fn run(&self) -> Option<Value> {
    match self.funcs.get("main") {
      Some(main) => main.execute(&vec![], &self.funcs, &mut Context::from(main)),
      None => panic!("No main function found"),
    }
  }

  fn parse(&mut self) -> Result<(), ParseError> {
    match parse(&self.file) {
      Ok(funcs) => {
        self.funcs = funcs;
        return Ok(());
      }
      Err(e) => Err(e),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Path, Program, Value};
  use std::convert::TryFrom;

  #[test]
  #[should_panic]
  fn test_missing_main() {
    let program = Program::try_from(Path::new("tests/samples/missing_main.rs")).unwrap();
    program.run();
  }

  #[test]
  fn test_empty_main() {
    let program = Program::try_from(Path::new("tests/samples/empty_main.rs")).unwrap();
    assert!(program.run().is_none());
  }

  #[test]
  fn test_return_in_main() {
    let program =
      Program::try_from(Path::new("tests/samples/return_in_main.rs")).unwrap();
    assert!(match program.run() {
      Some(value) => match value {
        Value::Int(3982) => true,
        _ => false,
      },
      None => false,
    })
  }

  #[test]
  fn test_if_statement() {
    let program =
      Program::try_from(Path::new("tests/samples/if_statement_true.rs")).unwrap();
    assert_eq!(program.run().unwrap(), Value::Int(5))
  }

  #[test]
  fn test_if_else() {
    let program = Program::try_from(Path::new("tests/samples/if_else.rs")).unwrap();
    assert_eq!(program.run().unwrap(), Value::Int(2))
  }

  #[test]
  fn test_assign() {
    let program = Program::try_from(Path::new("tests/samples/assign.rs")).unwrap();
    assert_eq!(program.run().unwrap(), Value::Int(6))
  }

  #[test]
  fn test_fibbonaci_recursive() {
    let program =
      Program::try_from(Path::new("tests/samples/fibbonaci_recursive.rs")).unwrap();
    assert_eq!(program.run().unwrap(), Value::Int(34))
  }
}
