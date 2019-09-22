use std::collections::HashMap;

#[derive(Debug)]
pub struct Scope{
    vars: HashMap<String, String>
}

impl Scope{
    pub fn new(mut args: Vec<String>) -> Scope{
        let mut vars = HashMap::new();
        vars.reserve(args.len());
        for arg in args.drain(..){
            if vars.contains_key(&arg){
                panic!("Duplicate argument");
            }
            //TODO: Don't use arg arg
            vars.insert(arg.clone(), arg);
        }
        Scope{ vars: vars }
    }
}
