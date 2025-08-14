use crate::{ast::expression_types::ExpressionType, token::token::Token};

#[derive(Debug, PartialEq)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<Option<ExpressionType>>,
    pub operator: String,
    pub right: Box<Option<ExpressionType>>,
}

impl InfixExpression {
    pub fn new(
        token: Token,
        left: Box<Option<ExpressionType>>,
        operator: String,
        right: Box<Option<ExpressionType>>,
    ) -> Self {
        InfixExpression {
            token,
            left,
            operator,
            right,
        }
    }

    pub fn string(&self) -> String {
        let left_str = if let Some(ref expr) = *self.left {
            expr.string()
        } else {
            "".to_string()
        };
        let right_str = if let Some(ref expr) = *self.right {
            expr.string()
        } else {
            "".to_string()
        };
        format!("({} {} {})", left_str, self.operator, right_str)
    }
}
