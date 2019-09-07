lalrpop_mod!(pub expr);

pub mod expr_parser{
    #[derive(Debug)]
    pub enum Expr{
        Number(i32),
        Op(Box<Expr>, Opcode, Box<Expr>)
    }
    
    #[derive(Debug)]
    pub enum Opcode{
        Mul, Div, Add, Sub
    }
    pub fn parse(s: &str) -> &str{
        debug_print!(crate::parsing::expr::ExprParser::new().parse(s).unwrap());
        "h"
    }
}
