use crate::{parser::ParseError, token::Token};

pub struct Program {
    pub statements: Vec<Node>,
    pub errors: Vec<ParseError>,
}

pub struct Node {
    pub token: Token,
}

impl Node {
    pub fn new(token: Token) -> Self {
        Node { token }
    }
}

trait Literal {
    fn token_literal(&self) -> String;
}

pub trait Statement: Literal {
    fn identifier(&self) -> Option<String>;
    fn expression(&self) -> Option<String>;
}

pub trait Expression: Literal {
    fn value(&self) -> Option<String>;
}

pub struct LetStatement {
    pub token: Token,
    pub identifier: Option<Identifier>,
}

impl LetStatement {
    pub fn new(token: Token) -> Self {
        LetStatement {
            token,
            identifier: None,
        }
    }
}

impl Literal for LetStatement {
    fn token_literal(&self) -> String {
        "let".to_string()
    }
}

pub struct Identifier {
    pub token: Token,
}

impl Expression for Identifier {
    fn value(&self) -> Option<String> {
        match &self.token {
            Token::Ident(ident) => Some(ident.clone()),
            _ => None,
        }
    }
}

impl Literal for Identifier {
    fn token_literal(&self) -> String {
        match &self.token {
            Token::Ident(ident) => ident.clone(),
            _ => "identifier".to_string(),
        }
    }
}

impl Identifier {
    pub fn new(token: Token) -> Self {
        Identifier { token }
    }
}
