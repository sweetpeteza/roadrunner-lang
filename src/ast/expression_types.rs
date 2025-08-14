use crate::ast::{identifier::Identifier, integer_literal::IntegerLiteral, prefix_expression::PrefixExpression, traits::Node};

#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    IntegerLiteral(IntegerLiteral),
    Identifier(Identifier),
    ExpressionStatement(Box<ExpressionType>),
    PrefixExpression(PrefixExpression),
}

impl ExpressionType {
    pub fn string(&self) -> String {
        match self {
            ExpressionType::IntegerLiteral(integer_literal) => integer_literal.string(),
            ExpressionType::Identifier(identifier) => identifier.string(),
            ExpressionType::ExpressionStatement(expression_statement) => expression_statement.string(),
            ExpressionType::PrefixExpression(prefix_expression) => prefix_expression.string(),
        }
    }
}
