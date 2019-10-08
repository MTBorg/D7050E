use std::{collections::HashMap, fs::File, io::prelude::*, path::Path};

use crate::{
  context::Context, func::Func, parsing::file_parser::parse, value::Value,
  variable::Variable,
};

pub struct Program {
  pub funcs: HashMap<String, Func>,
}

impl From<&Path> for Program {
  fn from(path: &Path) -> Self {
    let mut file = match File::open(path) {
      Ok(file) => file,
      Err(e) => panic!("Could not open input file: {}", e),
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
      Ok(_) => (),
      Err(e) => panic!("Could not read input file: {}", e),
    }

    match parse(s) {
      Ok(funcs) => Program { funcs: funcs },
      Err(e) => panic!("Failed to parse program: {:?}", e),
    }
  }
}

impl Program {
  pub fn run(&self) -> Option<Value> {
    match self.funcs.get("main") {
      Some(main) => main.execute(&vec![], &self.funcs, &mut Context::from(main)),
      None => panic!("No main function found"),
    }
  }

  pub fn get_main_context(&self) -> Option<Context<Variable>> {
    match self.funcs.get("main") {
      Some(main) => Some(Context::from(main)),
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Path, Program, Value};

  #[test]
  #[should_panic]
  fn test_missing_main() {
    Program::from(Path::new("tests/samples/missing_main.rs")).run();
  }

  #[test]
  fn test_empty_main() {
    assert!(Program::from(Path::new("tests/samples/empty_main.rs"))
      .run()
      .is_none());
  }

  #[test]
  fn test_return_in_main() {
    assert!(
      match Program::from(Path::new("tests/samples/return_in_main.rs")).run() {
        Some(value) => match value {
          Value::Int(3982) => true,
          _ => false,
        },
        None => false,
      }
    )
  }

  #[test]
  fn test_if_statement() {
    assert_eq!(
      Program::from(Path::new("tests/samples/if_statement_true.rs"))
        .run()
        .unwrap(),
      Value::Int(5)
    )
  }

  #[test]
  fn test_if_else() {
    assert_eq!(
      Program::from(Path::new("tests/samples/if_else.rs"))
        .run()
        .unwrap(),
      Value::Int(2)
    )
  }

  #[test]
  fn test_assign() {
    assert_eq!(
      Program::from(Path::new("tests/samples/assign.rs"))
        .run()
        .unwrap(),
      Value::Int(6)
    )
  }

  #[test]
  fn test_fibbonaci_recursive() {
    assert_eq!(
      Program::from(Path::new("tests/samples/fibbonaci_recursive.rs"))
        .run()
        .unwrap(),
      Value::Int(34)
    )
  }
}
