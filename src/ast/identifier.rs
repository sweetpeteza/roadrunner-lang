use crate::ast::traits::Node;

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
        self.value.clone()
    }
}
