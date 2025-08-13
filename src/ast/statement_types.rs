use crate::ast::{let_statement::LetStatement, return_statement::ReturnStatement};

#[derive(PartialEq, Debug)]
pub enum StatementType {
    Let(LetStatement),
    Return(ReturnStatement),
}
