use crate::{
    scope::Scope,
    value::Value,
    variable::Variable
};

#[derive(Debug)]
pub struct Context{
    scopes: Vec<Scope>
}

impl Context{
    pub fn new() -> Context{
        Context{scopes: vec!()} 
    }

    pub fn push(&mut self, scope: Scope){
        self.scopes.push(scope);
    }

    pub fn insert_variable(&mut self, id: String, val: Value){
        match (*self).scopes.iter_mut().last(){
            Some(scope) => (*scope).vars.insert(id.clone(), Variable{name: id, value: val}),
            None => panic!("Inserting into empty scope")
        };
    }

    pub fn get_variable(&self, var: String) -> Option<&Variable>{
        for scope in self.scopes.iter().rev().last(){
            match (*scope).vars.get(&var){
                Some(ref mut var) => { return Some(&var); },
                None => (),
            };
        }
        None
    }
}
