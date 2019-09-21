use std::collections::HashMap;

use crate::{
    func_param::FuncParam,
    node::Node,
    interpreter::eval,
    types::Context,
};

#[derive(Debug)]
pub struct FuncDec{
    pub name: String,
    pub params: Vec<FuncParam>,
    pub ret_type: String,
    pub body_start: Node
}

impl FuncDec {
    pub fn execute(&self, funcs: &HashMap<String, FuncDec>){
        println!("Executing {}", self.name);
            
        let mut c: Context = Context::new();
        eval(&self.body_start, &mut c, &funcs);
    }
}
