use crate::ast::{let_statement::LetStatement, return_statement::ReturnStatement, traits::Expression};

#[derive(PartialEq, Debug)]
pub enum StatementType<E> where E : Expression {
    Let(LetStatement<E>),
    Return(ReturnStatement<E>),
}

