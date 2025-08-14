use crate::ast::{identifier::Identifier, integer_literal::IntegerLiteral};

#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    IntegerLiteral(IntegerLiteral),
    Identifier(Identifier),
    ExpressionStatement(Box<ExpressionType>),
}
