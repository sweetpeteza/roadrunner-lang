use super::traits::{Node, Statement};

pub enum StatementType {
    Let(LetStatement),
}

pub struct LetStatement {
    pub name: String,
}

impl LetStatement {
    pub fn new(name: String) -> Self {
        LetStatement { name }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        "let".to_string()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

// pub struct ReturnStatement {
//     pub value: String,
// }
//
// impl Node for ReturnStatement {
//     fn token_literal(&self) -> String {
//         "return".to_string()
//     }
// }
//
// impl Statement for ReturnStatement {
//     fn statement_node(&self) {}
// }
