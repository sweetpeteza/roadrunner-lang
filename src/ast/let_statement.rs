use crate::{
    ast::{expression_types::ExpressionType, identifier::Identifier, traits::Node},
    token::token::Token,
};

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<ExpressionType>,
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Option<ExpressionType>) -> Self {
        LetStatement { token, name, value }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.to_literal()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push(' ');
        out.push_str(&self.name.token_literal());
        out.push_str(" = ");
        out.push_str(&match &self.value {
            Some(expression) => match expression {
                ExpressionType::Identifier(identifier) => identifier.string(),
                ExpressionType::IntegerLiteral(integer_literal) => integer_literal.string(),
                // Add other expression types as needed
                _ => "Expression not implemented".to_string(),
            },
            None => "nil".to_string(),
        });

        out.push(';');

        out
    }
}
