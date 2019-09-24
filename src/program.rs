use std::{collections::HashMap, fs::File, path::Path, io::prelude::*};

use crate::{
    parsing::file_parser::parse,
    func::Func,
    value::Value,
    context::Context,
};

pub struct Program{
    funcs: HashMap<String, Func>
}

impl From<&Path> for Program{
  fn from(path: &Path) -> Self{
    let mut file = match File::open(path) {
      Ok(file) => file,
      Err(e) => panic!("Could not open input file: {}", e),
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
      Ok(_) => (),
      Err(e) => panic!("Could not read input file: {}", e),
    }

    match parse(s){
        Ok(funcs) => Program{ funcs: funcs },
        Err(e) => panic!("Failed to parse program: {:?}", e)
    }
  }
}

impl Program{
  pub fn run(&self) -> Option<Value> {
    let mut context = Context::new();
    match self.funcs.get("main") {
      Some(main) => main.execute(&vec![], &self.funcs, &mut context),
      None => panic!("No main function found"),
    }
  }
}
