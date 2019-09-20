use crate::{
    func_param::FuncParam,
    node::Node,
    interpreter::eval
};

#[derive(Debug)]
pub struct FuncDec{
    pub name: String,
    pub params: Vec<FuncParam>,
    pub ret_type: String,
    pub body_start: Node
}

impl FuncDec {
    pub fn execute(&self){
        println!("Executing {}", self.name);

        eval(&self.body_start);
    }
}
