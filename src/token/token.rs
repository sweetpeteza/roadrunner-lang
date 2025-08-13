use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(i32),
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Eq,
    NotEq,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Illegal => write!(f, "Illegal"),
            Token::Eof => write!(f, "EOF"),
            Token::Ident(ident) => write!(f, "Ident({})", ident),
            Token::Int(value) => write!(f, "Int({})", value),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::LessThan => write!(f, "<"),
            Token::GreaterThan => write!(f, ">"),
            Token::Eq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Lparen => write!(f, "("),
            Token::Rparen => write!(f, ")"),
            Token::Lbrace => write!(f, "{{"),
            Token::Rbrace => write!(f, "}}"),
            Token::Function => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
        }
    }
}

pub const KEYWORDS: [(&str, Token); 7] = [
    ("fn", Token::Function),
    ("let", Token::Let),
    ("true", Token::True),
    ("false", Token::False),
    ("if", Token::If),
    ("else", Token::Else),
    ("return", Token::Return),
];

pub fn lookup_ident(ident: &str) -> Token {
    for &(key, ref value) in &KEYWORDS {
        if key == ident {
            return value.clone();
        }
    }
    Token::Ident(ident.to_string())
}
