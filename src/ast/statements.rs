use crate::{ast::traits::Expression};

use super::traits::{Node, Statement};

#[derive(PartialEq, Debug)]
pub enum StatementType<E: Expression> {
    Let(LetStatement<E>),
    Return(ReturnStatement<E>),
}

#[derive(Debug, PartialEq)]
pub struct LetStatement<E: Expression> {
    pub name: Identifier,
    pub value: Option<E>,
}

impl<E> LetStatement<E>
where
    E: Expression,
{
    pub fn new(name: Identifier, value: Option<E>) -> Self {
        LetStatement { name, value }
    }
}

impl<E> Node for LetStatement<E>
where
    E: Expression,
{
    fn token_literal(&self) -> String {
        "let".to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push(' ');
        out.push_str(&self.name.token_literal());
        out.push_str(" = ");
        out.push_str(&self.name.value);

        out.push(';');

        out
    }
}

impl<E> Statement for LetStatement<E>
where
    E: Expression,
{
    fn statement_node(&self) {}
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement<E: Expression> {
    pub return_value: Option<E>,
}

impl<E> ReturnStatement<E>
where
    E: Expression,
{
    pub fn new(return_value: Option<E>) -> Self {
        ReturnStatement {
            return_value,
        }
    }
}

impl<E> Node for ReturnStatement<E>
where
    E: Expression,
{
    fn token_literal(&self) -> String {
        "return".to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push(';');
        out
    }
}

impl<E> Statement for ReturnStatement<E>
where
    E: Expression,
{
    fn statement_node(&self) {}
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub value: String,
}

impl Identifier {
    pub fn new(value: String) -> Self {
        Identifier { value }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.value.clone()
    }

    fn string(&self) -> String {
        todo!()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

pub struct ExpressionStatement {
    pub expression: Option<Box<dyn Expression>>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        if let Some(ref expr) = self.expression {
            expr.token_literal()
        } else {
            "".to_string()
        }
    }

    fn string(&self) -> String {
        if let Some(ref expr) = self.expression {
            expr.string()
        } else {
            "".to_string()
        }
    }
}
