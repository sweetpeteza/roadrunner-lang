use crate::ast::traits::{Expression, Node};

pub struct ExpressionStatement {
    pub expression: Option<Box<dyn Expression>>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        if let Some(ref expr) = self.expression {
            expr.token_literal()
        } else {
            "".to_string()
        }
    }

    fn string(&self) -> String {
        if let Some(ref expr) = self.expression {
            expr.string()
        } else {
            "".to_string()
        }
    }
}
