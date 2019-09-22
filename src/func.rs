use std::collections::HashMap;

use crate::{
    func_param::FuncParam,
    node::Node,
    interpreter::eval,
    types::Context,
    scope::Scope
};

#[derive(Debug)]
pub struct FuncDec{
    pub name: String,
    pub params: Vec<FuncParam>,
    pub ret_type: String,
    pub body_start: Node
}

impl FuncDec {
    pub fn execute(&self, args: &Vec<String>, funcs: &HashMap<String, FuncDec>){
        println!("Executing {}", self.name);
            
        self.validate_arguments(args);
        let mut context: Context = Context::new();
        context.push(Scope::new((*args).clone()));

        eval(&self.body_start, &mut context, &funcs);
    }

    fn validate_arguments(&self, args: &Vec<String>){
        if args.len() < self.params.len(){
            let mut error_msg = "Missing parameter ".to_string() +
                &self.params[args.len()].name;
            for param in args.len()+1..self.params.len(){
                error_msg.push_str(", ");
                error_msg += &self.params[param].name;
            }

            error_msg.push_str(" to function ");
            error_msg += &self.name;
            panic!(error_msg);
        } else if args.len() > self.params.len(){
            panic!("Unexpected argument");
        }
    }
}
