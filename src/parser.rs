use crate::{ast::Program, lexer::Lexer, token::Token};

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::Illegal, // Initialize with an illegal token
            peek_token: Token::Illegal,    // Initialize with an illegal token
        };
        parser.next_token(); // Load the first token
        parser.next_token(); // Load the second token
        parser
    }

    pub fn parse_program(&self) -> Result<Program, ParseError> {
        // Placeholder for parsing logic
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token != Token::EOF {
            match self.parse_statement() {
                Ok(statement) => program.statements.push(statement),
                Err(e) => return Err(e), // Return on error
            }
            self.next_token(); // Move to the next token
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<ast::Node, ParseError> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            // Add more cases for different statement types
            _ => Err(ParseError {
                message: "Unknown statement type".to_string(),
                token: self.current_token.clone(),
            }),
        }
    }

    fn parse_let_statement(&mut self) -> Result<ast::Node, ParseError> {
        // Placeholder for parsing a let statement
        if let Token::Let = self.current_token {
            let statement = ast::LetStatement::new(self.current_token.clone());


        } else {
            Err(ParseError {
                message: "Expected 'let' token".to_string(),
                token: self.current_token.clone(),
            })
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.peek_token == expected {
            self.next_token();
            Ok(())
        } else {
            Err(ParseError {
                message: format!("Expected token {:?}, got {:?}", expected, self.peek_token),
                token: self.peek_token.clone(),
            })
        }
    }
}

pub struct ParseError {
    message: String,
    token: Token,
}
