use crate::{
    ast::{expression_types::ExpressionType, traits::Node},
    token::token::Token,
};

pub struct ExpressionStatement {
    pub token: Token, // first token of the expression
    pub expression: Option<ExpressionType>,
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Option<ExpressionType>) -> Self {
        ExpressionStatement { token, expression }
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.to_literal()
    }

    fn string(&self) -> String {
        if let Some(expr) = &self.expression {
            expr.string()
        } else {
            "".to_string()
        }
    }
}
