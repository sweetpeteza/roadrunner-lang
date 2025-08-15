use crate::{
    ast::{block_statement::BlockStatement, expression_types::ExpressionType, traits::Node},
    token::Token,
};

#[derive(Debug, PartialEq)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<Option<ExpressionType>>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl IfExpression {
    pub fn new(
        token: Token,
        condition: Box<Option<ExpressionType>>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    ) -> Self {
        Self {
            token,
            condition,
            consequence,
            alternative,
        }
    }
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.to_literal()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("if");
        if let Some(condition) = &self.condition.as_ref() {
            out.push_str(&condition.string());
        }
        out.push_str(" ");
        out.push_str(&self.consequence.string());
        if self.alternative.is_some() {
            out.push_str("else ");
            out.push_str(&self.consequence.string());
        }

        out
    }
}
