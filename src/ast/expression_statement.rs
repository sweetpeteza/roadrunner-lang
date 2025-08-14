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
            match expr {
                ExpressionType::Identifier(identifier) => identifier.string(),
                ExpressionType::IntegerLiteral(integer_literal) => integer_literal.string(),
                _ => {
                    // Handle other expression types as needed
                    // For now, we will just return a placeholder string
                    "Expression not implemented".to_string()
                } // ExpressionType::BooleanLiteral(boolean_literal) => boolean_literal.string(),
                  // ExpressionType::PrefixExpression(prefix_expr) => prefix_expr.string(),
                  // ExpressionType::InfixExpression(infix_expr) => infix_expr.string(),
                  // ExpressionType::IfExpression(if_expr) => if_expr.string(),
                  // ExpressionType::FunctionLiteral(func_literal) => func_literal.string(),
                  // ExpressionType::CallExpression(call_expr) => call_expr.string(),
                  // ExpressionType::ArrayLiteral(array_literal) => array_literal.string(),
                  // ExpressionType::IndexExpression(index_expr) => index_expr.string(),
            }
        } else {
            "".to_string()
        }
    }
}
