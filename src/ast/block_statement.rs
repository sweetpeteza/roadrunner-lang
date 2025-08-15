use crate::{
    ast::{statement_types::StatementType, traits::Node},
    token::Token,
};

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<StatementType>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.to_literal()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for stmt in self.statements.iter() {
            match stmt {
                StatementType::Let(let_stmt) => {
                    out.push_str(&let_stmt.string());
                }
                StatementType::Return(ret_stmt) => {
                    out.push_str(&ret_stmt.string());
                }
                StatementType::Expr(expr) => match expr {
                    Some(e) => out.push_str(&e.string()),
                    None => {}
                },
                StatementType::Block(block) => {
                    out.push_str(&block.string())
                }
            };
        }

        out
    }
}
