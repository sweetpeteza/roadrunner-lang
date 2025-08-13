use crate::{
    ast::traits::{Expression, Node, Statement},
    token::token::Token,
};

pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<dyn Expression>>,
}

impl PartialEq for ReturnStatement {
    fn eq(&self, other: &Self) -> bool {
        self.return_value.as_ref().map(|v| v.string())
            == other.return_value.as_ref().map(|v| v.string())
    }
}

impl std::fmt::Debug for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReturnStatement")
            .field(
                "return_value",
                &self.return_value.as_ref().map(|v| v.string()),
            )
            .finish()
    }
}

impl ReturnStatement {
    pub fn new(token: Token, return_value: Option<Box<dyn Expression>>) -> Self {
        ReturnStatement {
            token,
            return_value,
        }
    }
}

impl Node for ReturnStatement {
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

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}
