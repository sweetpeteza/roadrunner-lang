use crate::{ast::traits::Expression, token::token::Token};

use super::traits::{Node, Statement};

#[derive(PartialEq, Debug)]
pub enum StatementType {
    Let(LetStatement),
    Return(ReturnStatement),
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub name: Identifier,
}

impl LetStatement {
    pub fn new(name: Identifier) -> Self {
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

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub token: Token,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        "return".to_string()
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub value: String,
}

impl Identifier {
    pub fn new(value: String) -> Self {
        Identifier {
            value
        }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.value.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}
