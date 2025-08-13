use crate::{ast::traits::Expression};

use super::traits::{Node, Statement};

#[derive(PartialEq, Debug)]
pub enum StatementType<E> where E : Expression {
    Let(LetStatement<E>),
    Return(ReturnStatement<E>),
}

