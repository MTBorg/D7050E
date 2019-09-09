use crate::funcParam::FuncParam;

#[derive(Debug)]
pub struct FuncDec{
    pub name: String,
    pub params: Vec<FuncParam>,
    pub ret_type: String
}
