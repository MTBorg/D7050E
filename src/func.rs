use crate::funcParam::FuncParam;

#[derive(Debug)]
pub struct Func{
    pub name: String,
    pub params: Vec<FuncParam>,
    pub ret_type: String
}
