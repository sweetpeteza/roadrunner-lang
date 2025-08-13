use crate::ast::{
    identifier::Identifier,
    traits::{Expression, Node, Statement},
};

pub struct LetStatement {
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl PartialEq for LetStatement {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.value.as_ref().map(|v| v.string()) == other.value.as_ref().map(|v| v.string())
    }
}

impl std::fmt::Debug for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LetStatement")
            .field("name", &self.name)
            .field("value", &self.value.as_ref().map(|v| v.string()))
            .finish()
    }
}

impl LetStatement {
    pub fn new(name: Identifier, value: Option<Box<dyn Expression>>) -> Self {
        LetStatement { name, value }
    }
}

impl Node for LetStatement {
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

impl Statement for LetStatement {
    fn statement_node(&self) {}
}
