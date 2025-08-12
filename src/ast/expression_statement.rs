use crate::ast::traits::{Expression, Node};

pub struct ExpressionStatement<E> where E: Expression {
    pub expression: Option<Box<E>>,
}

impl<E> Node for ExpressionStatement<E> where E: Expression {
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
