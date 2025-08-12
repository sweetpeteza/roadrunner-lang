use crate::ast::{identifier::Identifier, traits::{Expression, Node, Statement}};

#[derive(Debug, PartialEq)]
pub struct LetStatement<E> where E : Expression {
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

