use crate::{
    ast::{block_statement::BlockStatement, identifier::Identifier, traits::Node},
    token::Token,
};

#[derive(Debug, PartialEq)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.to_literal()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        let params = self
            .parameters
            .iter()
            .map(|p| p.string())
            .collect::<Vec<String>>();

        out.push_str(&self.token.to_literal());
        out.push_str("(");
        out.push_str(params.join(", ").as_str());
        out.push_str(")");
        out.push_str(&self.body.string());

        out
    }
}
