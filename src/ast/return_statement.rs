use crate::{
    ast::{expression_types::ExpressionType, traits::Node},
    token::Token,
};

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<ExpressionType>,
}

impl ReturnStatement {
    pub fn new(token: Token, return_value: Option<ExpressionType>) -> Self {
        ReturnStatement {
            token,
            return_value,
        }
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.to_literal()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push(';');
        out
    }
}
