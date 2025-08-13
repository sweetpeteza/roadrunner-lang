use crate::ast::{
    let_statement::LetStatement, return_statement::ReturnStatement, traits::Expression,
};

#[derive(Debug)]
pub enum StatementType {
    Let(LetStatement),
    Return(ReturnStatement),
    Expr(Box<dyn Expression + 'static>),
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
