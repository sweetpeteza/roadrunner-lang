use crate::{ast::{expression_types::ExpressionType, traits::Node}, token::token::Token};

#[derive(Debug, PartialEq)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right:Box<Option<ExpressionType>>,
}

impl PrefixExpression {
    pub fn new(token: Token, operator: String, right: Box<Option<ExpressionType>>) -> Self {
        PrefixExpression { token, operator, right }
    }
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.to_literal()
    }

    fn string(&self) -> String {
        let right_str = if let Some(ref expr) = *self.right {
            expr.string()
        } else {
            "".to_string()
        };
        format!("({}{})", self.operator, right_str)
    }
}
