use rstest::rstest;

use super::statement_types::StatementType;
use super::traits::Node;

pub struct Program {
    pub statements: Vec<StatementType>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if let Some(first_statement) = self.statements.first() {
            match first_statement {
                StatementType::Let(let_stmt) => let_stmt.token_literal(),
                StatementType::Return(return_stmt) => return_stmt.token_literal(),
                StatementType::Expr(expr_stmt) => expr_stmt.token_literal(),
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
                StatementType::Expr(expr_stmt) => {
                    out.push_str(&expr_stmt.string());
                }
            }
        }
        out
    }
}

#[rstest]
fn test_program_string() {
    use crate::ast::identifier::Identifier;
    use crate::ast::let_statement::LetStatement;
    use crate::token::token::Token;

    const EXPECTED: &str = "let myVar = anotherVar;";

    let program = Program {
        statements: vec![StatementType::Let(LetStatement {
            token: Token::Let,
            name: Identifier {
                value: "myVar".to_string(),
            },
            value: Some(Box::new(Identifier {
                value: "anotherVar".to_string(),
            })),
        })],
    };

    assert_eq!(program.string(), EXPECTED);
}
