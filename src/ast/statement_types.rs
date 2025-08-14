use crate::ast::{
    expression_types::ExpressionType, let_statement::LetStatement,
    return_statement::ReturnStatement,
};

#[derive(Debug)]
pub enum StatementType {
    Let(LetStatement),
    Return(ReturnStatement),
    Expr(Option<ExpressionType>),
}

impl PartialEq for StatementType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Let(l), Self::Let(r)) => l == r,
            (Self::Return(l), Self::Return(r)) => l == r,
            (Self::Expr(l), Self::Expr(r)) => l == r,
            _ => false,
        }
    }
}
