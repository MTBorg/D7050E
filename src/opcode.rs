#[derive(Debug, PartialEq, Clone)]
pub enum Opcode {
  Mul,
  Div,
  Add,
  Sub,
  Eq,
  Neq,
  And,
  Or,
  Geq,
  Leq,
  Gneq,
  Lneq,
}

impl Opcode {
  pub fn to_str(&self) -> &'static str {
    match self {
      Opcode::Mul => "*",
      Opcode::Div => "/",
      Opcode::Add => "+",
      Opcode::Sub => "-",
      Opcode::Eq => "==",
      Opcode::Neq => "!=",
      Opcode::And => "&&",
      Opcode::Or => "||",
      Opcode::Geq => ">=",
      Opcode::Leq => "<=",
      Opcode::Gneq => ">",
      Opcode::Lneq => "<",
    }
  }
}
