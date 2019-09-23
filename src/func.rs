use std::collections::HashMap;

use crate::{
    func_param::FuncParam,
    node::Node,
    interpreter::eval,
    context::Context,
    scope::Scope,
    variable::Variable
};

#[derive(Debug)]
pub struct FuncDec{
    pub name: String,
    pub params: Vec<FuncParam>,
    pub ret_type: String,
    pub body_start: Node
}

impl FuncDec {
    pub fn execute(&self, args: &Vec<Node>, funcs: &HashMap<String, FuncDec>, context: &mut Context){
        println!("Executing {}", self.name);
            
        self.validate_arguments(args);

        let mut _args: Vec<Variable> = vec!();
        for pair in (*args).iter().zip(self.params.iter()){
            let (node, param) = pair;
            let val = eval(node, context, funcs).to_value();
            match val{
                Ok(val) => {_args.push(Variable{name: param.name.clone(), value: val});},
                Err(e) => {panic!("aiwuhdiauwhd");}
            };
        }
        
        let mut context: Context = Context::new();
        context.push(Scope::from(_args));

        eval(&self.body_start, &mut context, &funcs);
    }

    fn validate_arguments(&self, args: &Vec<Node>){
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
