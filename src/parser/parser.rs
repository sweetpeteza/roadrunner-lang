use rstest::rstest;

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

#[derive(Clone, Debug)]
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
                Some(Ok(statement)) => {
                    program.statements.push(statement);
                }
                Some(Err(e)) => {
                    self.errors.push(e); // Collect errors
                }
                None => {}
            }
            self.next_token(); // Move to the next token
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Result<StatementType, ParseError>> {
        match self.current_token {
            Token::Let => Some(self.parse_let_statement()),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Result<StatementType, ParseError> {
        let name = if let Token::Ident(name) = self.peek_token.clone() {
            name
        } else {
            return Err(ParseError {
                message: "Expected identifier after 'let'".to_string(),
                token: self.current_token.clone(),
            });
        };

        self.next_token(); // Move past the identifier

        if self.peek_token != Token::Assign {
            return Err(ParseError {
                message: "Expected '=' after variable name".to_string(),
                token: self.current_token.clone(),
            });
        }

        while self.current_token != Token::Semicolon {
            self.next_token(); // Skip tokens until we reach a semicolon
        }

        // Here you would parse the value, but for simplicity, we will skip it
        // In a complete implementation, you would handle expressions here

        Ok(StatementType::Let(LetStatement::new(name)))
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    // fn expect_peek(&mut self, expected: Token) -> Result<(), ParseError> {
    //     if self.peek_token != expected {
    //         return Err(ParseError {
    //             message: format!("Expected token: {}, got: {}", expected, self.peek_token),
    //             token: self.peek_token.clone(),
    //         });
    //     }
    //     self.next_token(); // Move past the expected token
    //     Ok(())
    // }
}

#[rstest]
fn test_let_statements() {
    let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    let errors = parser
        .errors
        .iter()
        .filter(|e| !e.message.starts_with("TEMP:"))
        .collect::<Vec<&ParseError>>();

    errors.clone().into_iter().for_each(|e| {
        eprintln!("Error: {} at token {:?}", e.message, e.token);
    });

    assert_eq!(errors.len(), 0);
    assert_eq!(program.statements.len(), 3);
    assert_eq!(
        program.statements[0],
        StatementType::Let(LetStatement::new("x".to_string()))
    );
    assert_eq!(
        program.statements[1],
        StatementType::Let(LetStatement::new("y".to_string()))
    );
    assert_eq!(
        program.statements[2],
        StatementType::Let(LetStatement::new("foobar".to_string()))
    );
}

#[rstest]
fn test_broken_let_statements() {
    let input = "
        let x 5;
        let = 10;
        let 838383;
        ";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    parser.errors.clone().into_iter().for_each(|e| {
        eprintln!("Error: {} at token {:?}", e.message, e.token);
    });
    assert_eq!(parser.errors.len(), 3);
    assert_eq!(program.statements.len(), 0);

    let mut errors = parser.errors.into_iter();

    let first_error = errors.next().unwrap();
    assert_eq!(
        first_error.message,
        "Expected '=' after variable name".to_string()
    );

    let second_error = errors.next().unwrap();
    assert_eq!(
        second_error.message,
        "Expected identifier after 'let'".to_string()
    );

    let third_error = errors.next().unwrap();
    assert_eq!(
        third_error.message,
        "Expected identifier after 'let'".to_string()
    );
}
