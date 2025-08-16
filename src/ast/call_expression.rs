use crate::{
    ast::{expression_types::ExpressionType, traits::Node},
    token::Token,
};

#[derive(Debug, PartialEq)]
pub struct CallExpression {
    pub token: Token, // the '(' token
    pub function: Box<ExpressionType>,
    pub arguments: Vec<ExpressionType>,
}

impl CallExpression {
    pub fn new(
        token: Token,
        function: Box<ExpressionType>,
        arguments: Vec<ExpressionType>,
    ) -> CallExpression {
        CallExpression {
            token,
            function,
            arguments,
        }
    }
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.to_literal()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        let args = self
            .arguments
            .iter()
            .map(|arg| arg.string())
            .collect::<Vec<String>>()
            .join(", ");

        out.push_str(&self.function.string());
        out.push_str("(");
        out.push_str(&args);
        out.push_str(")");

        out
    }
}
