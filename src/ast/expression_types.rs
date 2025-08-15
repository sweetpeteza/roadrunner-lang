use crate::ast::{
    boolean_literal::BooleanLiteral, function_literal::FunctionLiteral, identifier::Identifier, if_expression::IfExpression, infix_expression::InfixExpression, integer_literal::IntegerLiteral, prefix_expression::PrefixExpression, traits::Node
};

#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    IntegerLiteral(IntegerLiteral),
    Identifier(Identifier),
    Statement(Box<ExpressionType>),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    BooleanLiteral(BooleanLiteral),
    If(IfExpression),
    Function(FunctionLiteral)
}

impl ExpressionType {
    pub fn string(&self) -> String {
        match self {
            ExpressionType::IntegerLiteral(integer_literal) => integer_literal.string(),
            ExpressionType::Identifier(identifier) => identifier.string(),
            ExpressionType::Statement(expression_statement) => expression_statement.string(),
            ExpressionType::Prefix(prefix_expression) => prefix_expression.string(),
            ExpressionType::Infix(infix_expression) => infix_expression.string(),
            ExpressionType::BooleanLiteral(boolean_literal) => boolean_literal.string(),
            ExpressionType::If(if_expression) => if_expression.string(),
            ExpressionType::Function(fn_expression) => fn_expression.string(),
        }
    }
}
