use crate::func_param::FuncParam;

#[derive(Debug)]
pub struct FuncDec{
    pub name: String,
    pub params: Vec<FuncParam>,
    pub ret_type: String
}
