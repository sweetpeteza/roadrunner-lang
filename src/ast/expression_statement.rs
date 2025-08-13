use crate::{ast::traits::{ExprStatement, Expression, Node, Statement}, token::token::Token};

pub struct ExpressionStatement {
    pub token: Token, // first token of the expression
    pub expression: Option<Box<dyn Expression>>,
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Option<Box<dyn Expression>>) -> Self {
        ExpressionStatement { token, expression }
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.to_literal()
    }

    fn string(&self) -> String {
        if let Some(ref expr) = self.expression {
            expr.string()
        } else {
            "".to_string()
        }
    }
}

impl Expression for ExpressionStatement {
    fn expression_node(&self) {
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {
    }
}

impl ExprStatement for ExpressionStatement {
    fn expression(&self) -> Option<&dyn Expression> {
        // self.expression.unwrap_or_else(|e| Some(Box::new(e)), None))
        self.expression.as_ref().map(|e| e.as_ref())
    }
}
