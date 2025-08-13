use crate::{ast::traits::{Expression, Node}, token::token::Token};

pub struct ExpressionStatement {
    pub token: Token, // first token of the expression
    pub expression: Option<Box<dyn Expression>>,
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Option<Box<dyn Expression>>) -> Self {
        ExpressionStatement { token, expression }
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.to_literal()
    }

    fn string(&self) -> String {
        if let Some(ref expr) = self.expression {
            expr.string()
        } else {
            "".to_string()
        }
    }
}
