use crate::{ast::traits::Node, token::Token};

#[derive(Debug, PartialEq)]
pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
}

impl BooleanLiteral {
    pub fn new(token: Token, value: bool) -> Self {
        BooleanLiteral { token, value }
    }
}

impl Node for BooleanLiteral {
    fn token_literal(&self) -> String {
        self.token.to_literal()
    }

    fn string(&self) -> String {
        self.value.to_string().clone()
    }
}

