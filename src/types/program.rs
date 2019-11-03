use std::{collections::HashMap, fs::File, io::prelude::*, path::Path};

use crate::{
  errors::parse_error::ParseError,
  parsing::file_parser::parse,
  types::{context::Context, func::Func, value::Value},
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
  #[allow(dead_code)]
  pub fn interpret(&self) -> Option<Value> {
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
  use crate::type_checker::type_check_program;
  use std::convert::TryFrom;

  #[test]
  #[should_panic]
  fn test_missing_main() {
    let program = Program::try_from(Path::new("tests/samples/missing_main.rs")).unwrap();
    program.interpret();
  }

  #[test]
  fn test_empty_main() {
    let program = Program::try_from(Path::new("tests/samples/empty_main.rs")).unwrap();
    assert!(program.interpret().is_none());
  }

  #[test]
  fn test_return_in_main() {
    let program =
      Program::try_from(Path::new("tests/samples/return_in_main.rs")).unwrap();
    assert!(match program.interpret() {
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
    type_check_program(&program).unwrap();
    assert_eq!(program.interpret().unwrap(), Value::Int(5))
  }

  #[test]
  fn test_if_else() {
    let program = Program::try_from(Path::new("tests/samples/if_else.rs")).unwrap();
    type_check_program(&program).unwrap();
    assert_eq!(program.interpret().unwrap(), Value::Int(2))
  }

  #[test]
  fn test_assign() {
    let program = Program::try_from(Path::new("tests/samples/assign.rs")).unwrap();
    type_check_program(&program).unwrap();
    assert_eq!(program.interpret().unwrap(), Value::Int(6))
  }

  #[test]
  fn test_fibbonaci_recursive() {
    let program =
      Program::try_from(Path::new("tests/samples/fibbonaci_recursive.rs")).unwrap();
    if let Err(e) = type_check_program(&program) {
      panic!("{:?}", e);
    }
    assert_eq!(program.interpret().unwrap(), Value::Int(34))
  }

  #[test]
  fn test_nested_function_calls() {
    let program =
      Program::try_from(Path::new("tests/samples/nested_function_calls.rs")).unwrap();
    type_check_program(&program).unwrap();
    assert_eq!(program.interpret().unwrap(), Value::Int(6))
  }

  #[test]
  fn type_inference_i32() {
    let program =
      Program::try_from(Path::new("tests/samples/type_inference_i32.rs")).unwrap();
    type_check_program(&program).unwrap();
    assert_eq!(program.interpret().unwrap(), Value::Int(34))
  }

  #[test]
  fn type_inference_bool() {
    let program =
      Program::try_from(Path::new("tests/samples/type_inference_bool.rs")).unwrap();
    type_check_program(&program).unwrap();
    assert_eq!(program.interpret().unwrap(), Value::Bool(true))
  }

  #[test]
  fn shadowing_return_shadowed() {
    let program =
      Program::try_from(Path::new("tests/samples/shadowing_return_shadowed.rs")).unwrap();
    type_check_program(&program).unwrap();
    assert_eq!(program.interpret().unwrap(), Value::Int(4))
  }

  #[test]
  fn shadowing_return_original() {
    let program =
      Program::try_from(Path::new("tests/samples/shadowing_return_original.rs")).unwrap();
    type_check_program(&program).unwrap();
    assert_eq!(program.interpret().unwrap(), Value::Int(14))
  }

  #[test]
  fn while_count_to_10() {
    let program =
      Program::try_from(Path::new("tests/samples/while_count_to_10.rs")).unwrap();
    type_check_program(&program).unwrap();
    assert_eq!(program.interpret().unwrap(), Value::Int(10))
  }

  #[test]
  fn while_shadowing_original_unaffected() {
    let program = Program::try_from(Path::new(
      "tests/samples/while_shadowing_original_unaffected.rs",
    ))
    .unwrap();
    type_check_program(&program).unwrap();
    assert_eq!(program.interpret().unwrap(), Value::Int(4))
  }

  #[test]
  fn while_does_not_enter_if_condition_false() {
    let program = Program::try_from(Path::new(
      "tests/samples/while_does_not_enter_if_condition_false.rs",
    ))
    .unwrap();
    type_check_program(&program).unwrap();
    assert_eq!(program.interpret().unwrap(), Value::Int(4))
  }

  #[test]
  #[should_panic]
  fn while_should_not_access_var_in_loop_from_outside() {
    let program = Program::try_from(Path::new(
      "tests/samples/while_should_not_access_var_in_loop_from_outside.rs",
    ))
    .unwrap();
    program.interpret();
  }

  #[test]
  #[should_panic]
  fn while_should_not_access_var_in_loop_from_previous_iteration() {
    let program = Program::try_from(Path::new(
      "tests/samples/while_should_not_access_variable_from_previous_iteration.rs",
    ))
    .unwrap();
    program.interpret();
  }
}
