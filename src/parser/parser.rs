use crate::{
    ast::{
        program::Program,
        statements::{LetStatement, StatementType},
    },
    lexer::lexer::Lexer,
    token::token::Token,
};

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: Vec<ParseError>,
}

pub struct ParseError {
    pub message: String,
    pub token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::Illegal, // Initialize with an illegal token
            peek_token: Token::Illegal,    // Initialize with an illegal token
            errors: Vec::new(),
        };

        parser.next_token(); // Load the first token
        parser.next_token(); // Load the second token

        parser
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.current_token != Token::Eof {
            match self.parse_statement() {
                Ok(statement) => program.statements.push(statement),
                Err(e) => self.errors.push(e), // Return on error
            }
            self.next_token(); // Move to the next token
        }
        program
    }

    fn parse_statement(&mut self) -> Result<StatementType, ParseError> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            _ => Err(ParseError {
                message: format!("Unexpected token: {}", self.current_token),
                token: self.current_token.clone(),
            }),
        }
    }

    fn parse_let_statement(&mut self) -> Result<StatementType, ParseError> {
        self.expect_peek(Token::Let).map_err(|_| ParseError {
            message: "Expected 'let' keyword".to_string(),
            token: self.peek_token.clone(),
        })?;

        let name = if let Token::Ident(name) = self.current_token.clone() {
            name
        } else {
            return Err(ParseError {
                message: "Expected identifier after 'let'".to_string(),
                token: self.current_token.clone(),
            });
        };

        self.next_token(); // Move past the identifier

        if self.current_token != Token::Assign {
            return Err(ParseError {
                message: "Expected '=' after variable name".to_string(),
                token: self.current_token.clone(),
            });
        }

        self.next_token(); // Move past '='

        // Here you would parse the value, but for simplicity, we will skip it
        // In a complete implementation, you would handle expressions here

        Ok(StatementType::Let(LetStatement::new(name)))
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.peek_token != expected {
            return Err(ParseError {
                message: format!("Expected token: {}, got: {}", expected, self.peek_token),
                token: self.peek_token.clone(),
            });
        }
        self.next_token(); // Move past the expected token
        Ok(())
    }
}
