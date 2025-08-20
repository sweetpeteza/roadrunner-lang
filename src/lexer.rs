use rstest::rstest;

use crate::token::{Token, lookup_ident};
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position..].chars().next().unwrap();
        }
        self.position = self.read_position;
        self.read_position += self.ch.len_utf8();
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position..].chars().next().unwrap()
        }
    }

    pub fn next_token(&mut self) -> Token {
        use crate::token::Token::*;
        self.skip_whitespace();

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Eq
                } else {
                    Assign
                }
            }
            '+' => Plus,
            '-' => Minus,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    NotEq
                } else {
                    Bang
                }
            }
            '*' => Asterisk,
            '<' => LessThan,
            '>' => GreaterThan,
            '/' => Slash,
            ',' => Comma,
            ';' => Semicolon,
            '(' => Lparen,
            ')' => Rparen,
            '{' => Lbrace,
            '}' => Rbrace,
            '\0' => Eof,
            _ if self.ch.is_alphabetic() || self.ch == '_' => {
                let ident = self.read_identifier();
                return lookup_ident(&ident);
            }
            _ if self.ch.is_ascii_digit() => {
                let literal = self.read_number();
                return Int(literal);
            }
            _ => Illegal,
        };

        self.read_char();

        token
    }

    fn read_identifier(&mut self) -> String {
        let start_position = self.position;
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }
        self.input[start_position..self.position].to_string()
    }

    fn read_number(&mut self) -> i64 {
        let start_position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[start_position..self.position]
            .parse::<i64>()
            .unwrap_or(0)
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}

#[rstest]
fn test_next_token_simple() {
    let input = "=+(){},;";
    let mut lexer = Lexer::new(input);
    use crate::token::Token::*;

    let tests = vec![
        Assign, Plus, Lparen, Rparen, Lbrace, Rbrace, Comma, Semicolon, Eof,
    ];

    for expected_token in tests {
        let token = lexer.next_token();
        assert_eq!(token, expected_token);
    }
}

#[rstest]
fn test_next_token_semicolon() {
    use crate::token::Token::*;
    let input = ";";
    let mut lexer = Lexer::new(input);

    let tests = vec![Semicolon, Eof];

    for expected_token in tests {
        let token = lexer.next_token();
        assert_eq!(token, expected_token);
    }
}

#[rstest]
fn test_next_token_equals() {
    use crate::token::Token::*;
    let input = "==;";
    let mut lexer = Lexer::new(input);

    let tests = vec![Eq, Semicolon, Eof];

    for expected_token in tests {
        let token = lexer.next_token();
        assert_eq!(token, expected_token);
    }
}

#[rstest]
fn test_next_token_not_equals() {
    use crate::token::Token::*;
    let input = "!=;";
    let mut lexer = Lexer::new(input);

    let tests = vec![NotEq, Semicolon, Eof];

    for expected_token in tests {
        let token = lexer.next_token();
        assert_eq!(token, expected_token);
    }
}

#[rstest]
fn test_next_token_double_char_tokens() {
    use crate::token::Token::*;
    let input = "== !=;";
    let mut lexer = Lexer::new(input);

    let tests = vec![Eq, NotEq, Semicolon, Eof];

    for expected_token in tests {
        let token = lexer.next_token();
        assert_eq!(token, expected_token);
    }
}

#[rstest]
fn test_next_token_statements() {
    use crate::token::Token::*;
    let input = "
    let five = 5;
    let ten = 10;

    let add = fn(x,y) {
        x + y
    };

    let result = add(five,ten);

    ";

    let mut lexer = Lexer::new(input);

    let tests = vec![
        Let,
        Ident("five".to_string()),
        Assign,
        Int(5),
        Semicolon,
        Let,
        Ident("ten".to_string()),
        Assign,
        Int(10),
        Semicolon,
        Let,
        Ident("add".to_string()),
        Assign,
        Function,
        Lparen,
        Ident("x".to_string()),
        Comma,
        Ident("y".to_string()),
        Rparen,
        Lbrace,
        Ident("x".to_string()),
        Plus,
        Ident("y".to_string()),
        Rbrace,
        Semicolon,
        Let,
        Ident("result".to_string()),
        Assign,
        Ident("add".to_string()),
        Lparen,
        Ident("five".to_string()),
        Comma,
        Ident("ten".to_string()),
        Rparen,
        Semicolon,
        Eof,
    ];

    for expected_token in tests {
        let token = lexer.next_token();

        match expected_token {
            Token::Ident(ident) => {
                assert_eq!(token, Token::Ident(ident));
            }
            Token::Int(value) => {
                assert_eq!(token, Token::Int(value));
            }
            _ => {
                assert_eq!(token, expected_token);
            }
        }
    }
}

#[rstest]
fn test_next_token_statements_and_operators() {
    use crate::token::Token::*;
    let input = "
    let five = 5;
    let ten = 10;

    let add = fn(x,y) {
        x + y
    };

    let result = add(five,ten);

    !-/*5; 
    5 < 10 > 5;
    
    if (5 < 10) {
        return true;
    } else {
        return false;
    }
";

    let mut lexer = Lexer::new(input);

    let tests = vec![
        Let,
        Ident("five".to_string()),
        Assign,
        Int(5),
        Semicolon,
        Let,
        Ident("ten".to_string()),
        Assign,
        Int(10),
        Semicolon,
        Let,
        Ident("add".to_string()),
        Assign,
        Function,
        Lparen,
        Ident("x".to_string()),
        Comma,
        Ident("y".to_string()),
        Rparen,
        Lbrace,
        Ident("x".to_string()),
        Plus,
        Ident("y".to_string()),
        Rbrace,
        Semicolon,
        Let,
        Ident("result".to_string()),
        Assign,
        Ident("add".to_string()),
        Lparen,
        Ident("five".to_string()),
        Comma,
        Ident("ten".to_string()),
        Rparen,
        Semicolon,
        Bang,
        Minus,
        Slash,
        Asterisk,
        Int(5),
        Semicolon,
        Int(5),
        LessThan,
        Int(10),
        GreaterThan,
        Int(5),
        Semicolon,
        If,
        Lparen,
        Int(5),
        LessThan,
        Int(10),
        Rparen,
        Lbrace,
        Return,
        True,
        Semicolon,
        Rbrace,
        Else,
        Lbrace,
        Return,
        False,
        Semicolon,
        Rbrace,
        Eof,
    ];

    for (num, expected_token) in tests.iter().enumerate() {
        let token = lexer.next_token();
        dbg!(num, &token, expected_token);

        match expected_token {
            Token::Ident(ident) => {
                assert_eq!(
                    token,
                    Token::Ident(ident.clone()),
                    "Test failed at index {}: expected {:?}, got {:?}",
                    num,
                    expected_token,
                    token
                );
            }
            Token::Int(value) => {
                assert_eq!(
                    token,
                    Token::Int(*value),
                    "Test failed at index {}: expected {:?}, got {:?}",
                    num,
                    expected_token,
                    token
                );
            }
            _ => {
                assert_eq!(
                    token,
                    expected_token.clone(),
                    "Test failed at index {}: expected {:?}, got {:?}",
                    num,
                    expected_token,
                    token
                );
            }
        }
    }
}
