use crate::ast::{traits::{Expression, Node, Statement}};

#[derive(Debug, PartialEq)]
pub struct ReturnStatement<E> where E : Expression {
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


