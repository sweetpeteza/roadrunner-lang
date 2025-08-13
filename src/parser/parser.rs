use std::{collections::HashMap, fmt::Debug};

use rstest::rstest;

use crate::{
    ast::{
        expression_statement::ExpressionStatement, identifier::Identifier, integer_literal::IntegerLiteral, let_statement::LetStatement, program::Program, return_statement::ReturnStatement, statement_types::StatementType, traits::Expression
    },
    lexer::lexer::Lexer,
    parser::function_types::{InfixParseFn, PrefixParseFn},
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

    fn parse_identifier(&self) -> Option<Box<dyn Expression>> {
        if let Token::Ident(ref ident) = self.current_token {
            Some(Box::new(Identifier::new(ident.clone())))
        } else {
            None
        }
    }

    fn parse_integer_literal(&self) -> Option<Box<dyn Expression>> {
        if let Token::Int(ref value) = self.current_token {
            Some(Box::new(IntegerLiteral::new(self.current_token.clone(), value.clone())))
        } else {
            None
        }
    }

    fn parse_statement(&mut self) -> Option<Result<StatementType, ParseError>> {
        match self.current_token {
            Token::Let => Some(self.parse_let_statement()),
            Token::Return => Some(self.parse_return_statement()),
            _ => Some(self.parse_expression_statement()),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<StatementType, ParseError> {
        let token = self.current_token.clone();

        let expression = self.parse_expression();

        let statement = StatementType::Expr(Box::new(ExpressionStatement::new(token, expression)));

        if self.peek_token == Token::Semicolon {
            self.next_token(); // Consume the semicolon
        }

        Ok(statement)
    }

    fn parse_expression(&mut self) -> Option<Box<dyn Expression>> {
        match &self.current_token {
            Token::Int(_) => self.parse_integer_literal(),
            Token::Ident(_) => self.parse_identifier(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Result<StatementType, ParseError> {
        let let_token = self.current_token.clone();

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

        Ok(StatementType::Let(LetStatement::new(
            let_token,
            Identifier::new(name),
            None,
        )))
    }

    fn parse_return_statement(&mut self) -> Result<StatementType, ParseError> {
        let return_token = self.current_token.clone();
        let return_statement = ReturnStatement::new(return_token, None);

        while self.current_token != Token::Semicolon {
            self.next_token(); // Skip tokens until we reach a semicolon
        }

        Ok(StatementType::Return(return_statement))
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
        StatementType::Let(LetStatement::new(
            Token::Let,
            Identifier::new("x".to_string()),
            None
        ))
    );
    assert_eq!(
        program.statements[1],
        StatementType::Let(LetStatement::new(
            Token::Let,
            Identifier::new("y".to_string()),
            None
        ))
    );
    assert_eq!(
        program.statements[2],
        StatementType::Let(LetStatement::new(
            Token::Let,
            Identifier::new("foobar".to_string()),
            None
        ))
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
    let _program = parser.parse_program();

    parser.errors.clone().into_iter().for_each(|e| {
        eprintln!("Error: {} at token {:?}", e.message, e.token);
    });
    assert_eq!(parser.errors.len(), 3);

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

#[rstest]
fn test_return_statements() {
    let input = "
        return 5;
        return 10;
        return 838383;
        ";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    let errors = parser.errors.into_iter();

    errors.clone().into_iter().for_each(|e| {
        eprintln!("Error: {} at token {:?}", e.message, e.token);
    });

    assert_eq!(errors.len(), 0);
    assert_eq!(program.statements.len(), 3);

    assert_eq!(
        program.statements[0],
        StatementType::Return(ReturnStatement::new(Token::Return, None))
    );
}

#[rstest]
fn test_identifier_expression() {
    let input = "foobar;";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    let errors = parser.errors.into_iter();

    errors.clone().into_iter().for_each(|e| {
        eprintln!("Error: {} at token {:?}", e.message, e.token);
    });

    assert_eq!(errors.len(), 0);
    assert_eq!(program.statements.len(), 1);

    if let Some(let_statement) = program.statements.first() {
        if let StatementType::Let(let_stmt) = let_statement {
            if let Some(expression) = &let_stmt.value {
                assert_eq!(expression.string(), "foobar");
            } else {
                panic!("Expected an expression in the let statement");
            }
        }
    }
}

#[rstest]
fn test_integer_literal_expression() {
    let input = "5;";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    let errors = parser.errors.into_iter();

    errors.clone().into_iter().for_each(|e| {
        eprintln!("Error: {} at token {:?}", e.message, e.token);
    });

    assert_eq!(errors.len(), 0);
    assert_eq!(program.statements.len(), 1);

    if let Some(int_literal) = program.statements.first() {
        if let StatementType::Int(int) = int_literal {
            assert_eq!(int.value, 5);
        } else {
            panic!("Expected an integer literal statement, found {:?}", int_literal);
        }
    }
}
