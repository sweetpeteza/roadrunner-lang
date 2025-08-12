use crate::ast::traits::Expression;

use super::statement_types::{StatementType};
use super::traits::{Node};

#[derive(Debug)]
pub struct Program<E> where E: Expression {
    pub statements: Vec<StatementType<E>>
}

impl<E> Program<E> where E: Expression {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
}

impl<E> Node for Program<E> where E : Expression {
    fn token_literal(&self) -> String {
        if let Some(first_statement) = self.statements.first() {
            match first_statement {
                StatementType::Let(let_stmt) => let_stmt.token_literal(),
                StatementType::Return(return_stmt) => return_stmt.token_literal(),
                // Add more cases for different statement types
            }
        } else {
            "".to_string()
        }
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for statement in &self.statements {
            match statement {
                StatementType::Let(let_stmt) => out.push_str(&let_stmt.string()),
                StatementType::Return(return_stmt) => out.push_str(&return_stmt.string()),
                // Add more cases for different statement types
            }
            out.push_str("\n");
        }
        out
    }
}
