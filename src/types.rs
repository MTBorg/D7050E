use std::collections::HashMap;

pub type Context = Vec<Scope>;
pub type Scope = HashMap<String, String>;
