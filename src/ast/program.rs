use super::statements::{StatementType};
use super::traits::{Node};

pub struct Program {
    pub statements: Vec<StatementType>
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }

    pub fn token_literal(&self) -> String {
        if let Some(first_statement) = self.statements.first() {
            match first_statement {
                StatementType::Let(let_stmt) => let_stmt.token_literal(),
                // Add more cases for different statement types
            }
        } else {
            "".to_string()
        }
    }
}
