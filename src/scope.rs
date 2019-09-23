use std::collections::HashMap;
use crate::variable::Variable;

#[derive(Debug)]
pub struct Scope{
    pub vars: HashMap<String, Variable>
}

impl Scope{
    pub fn new(mut vars: Vec<Variable>) -> Scope{
        let mut map = HashMap::new();
        map.reserve(vars.len());
        for var in vars.drain(..){
            if map.contains_key(&var.name){
                panic!("Duplicate argument");
            }
            //TODO: Don't use arg arg
            map.insert(var.name.clone(), var);
        }
        Scope{ vars: map }
    }
}
