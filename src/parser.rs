use std::collections::HashMap;
use std::fmt::Debug;

use rstest::rstest;

use crate::ast::expression_types::ExpressionType;
use crate::ast::infix_expression::InfixExpression;
use crate::ast::precedence::Precedence;
use crate::ast::prefix_expression::PrefixExpression;
use crate::{
    ast::{
        identifier::Identifier, integer_literal::IntegerLiteral, let_statement::LetStatement,
        program::Program, return_statement::ReturnStatement, statement_types::StatementType,
    },
    lexer::Lexer,
    token::Token,
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

    fn parse_expression_statement(&mut self) -> Result<StatementType, ParseError> {
        let expression = self.parse_expression(Precedence::Lowest);

        let statement = StatementType::Expr(expression);

        if self.peek_token == Token::Semicolon {
            self.next_token(); // Consume the semicolon
        }

        Ok(statement)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<ExpressionType> {
        // let prefix_fn = &self.prefix_parse_fns.get(&self.current_token);

        let prefix = match self.current_token.clone() {
            Token::Ident(_) => self.parse_identifier(),
            Token::Int(_) => self.parse_integer_literal(),
            Token::Bang | Token::Minus => self.parse_prefix_expression(),
            _ => None,
        };

        if prefix.is_none() {
            return None;
        }

        let mut left_expression = prefix;

        while &self.peek_token.clone() != &Token::Semicolon
            && precedence < self.get_precedence(&self.peek_token.clone())
        {
            left_expression = match self.peek_token.clone() {
                Token::Plus
                | Token::Minus
                | Token::Slash
                | Token::Asterisk
                | Token::Eq
                | Token::NotEq
                | Token::LessThan
                | Token::GreaterThan => {
                    self.next_token(); // Move past the infix operator
                    self.parse_infix_expression(left_expression?)
                }
                _ => {
                    return left_expression; // No infix function, return the left expression
                }
            };
        }

        left_expression
    }

    fn parse_infix_expression(&mut self, left: ExpressionType) -> Option<ExpressionType> {
        let current_token = self.current_token.clone();
        let operator = current_token.to_literal();
        let precedence = self.get_precedence(&current_token);

        self.next_token(); // Move past the operator

        let right = self.parse_expression(precedence);

        if right.is_none() {
            self.errors.push(ParseError {
                message: "Expected expression after infix operator".to_string(),
                token: self.current_token.clone(),
            });
            return None;
        }

        Some(ExpressionType::Statement(Box::new(ExpressionType::Infix(
            InfixExpression::new(
                current_token,
                Box::new(Some(left)),
                operator,
                Box::new(right),
            ),
        ))))
    }

    fn get_precedence(&self, token: &Token) -> Precedence {
        use crate::token::Token::*;
        match token {
            Eq | NotEq => Precedence::Equals,
            LessThan | GreaterThan => Precedence::LessGreater,
            Plus | Minus => Precedence::Sum,
            Asterisk | Slash => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<ExpressionType> {
        if let Token::Bang | Token::Minus = self.current_token {
            let current_token = self.current_token.clone();
            let operator = current_token.to_literal();
            self.next_token(); // Move past the operator

            let right = self.parse_expression(Precedence::Prefix);

            if right.is_none() {
                self.errors.push(ParseError {
                    message: "Expected expression after prefix operator".to_string(),
                    token: self.current_token.clone(),
                });
            }

            Some(ExpressionType::Statement(Box::new(ExpressionType::Prefix(
                PrefixExpression::new(current_token, operator, Box::new(right)),
            ))))
        } else {
            None
        }
    }

    fn parse_identifier(&mut self) -> Option<ExpressionType> {
        if let Token::Ident(ref ident) = self.current_token {
            Some(ExpressionType::Identifier(Identifier::new(ident.clone())))
        } else {
            None
        }
    }

    fn parse_integer_literal(&mut self) -> Option<ExpressionType> {
        if let Token::Int(ref value) = self.current_token {
            Some(ExpressionType::IntegerLiteral(IntegerLiteral::new(
                self.current_token.clone(),
                value.clone(),
            )))
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
            match &let_stmt.value {
                Some(ExpressionType::Identifier(ident)) => {
                    assert_eq!(ident.value, "foobar");
                }
                _ => {
                    panic!("Expected an expression in the let statement");
                }
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

    if let Some(integer_literal) = program.statements.first() {
        if let StatementType::Expr(int_literal) = integer_literal {
            match int_literal {
                Some(ExpressionType::IntegerLiteral(literal)) => {
                    assert_eq!(literal.value, 5);
                }
                _ => panic!("Expected an integer literal expression"),
            }
        }
    }
}

#[rstest]
#[case("!5;", "!", 5)]
#[case("-1;", "-", 1)]
fn test_parsing_prefix_expression(#[case] input: &str, #[case] operator: &str, #[case] value: i64) {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    let errors = parser.errors.into_iter();

    errors.clone().into_iter().for_each(|e| {
        eprintln!("Error: {} at token {:?}", e.message, e.token);
    });

    assert_eq!(errors.len(), 0);
    assert_eq!(program.statements.len(), 1);

    let mut statements = program.statements.iter();

    let first_statement = statements.next();

    let expr = match first_statement {
        Some(StatementType::Expr(expr)) => expr,
        _ => {
            panic!("Expected an expression statement");
        }
    };
    let expr = expr.as_ref().expect("Expected an expression statement");

    let expr = match expr {
        ExpressionType::Statement(expr_stmt) => expr_stmt.as_ref(),
        _ => panic!("Expected an expression statement"),
    };

    let prefix_expr = match expr {
        ExpressionType::Prefix(prefix_expr) => prefix_expr,
        _ => {
            panic!("Expected a prefix expression");
        }
    };

    assert_eq!(prefix_expr.operator, operator);

    let right_expr = prefix_expr.right.as_ref();

    let integer_literal = match right_expr {
        Some(ExpressionType::IntegerLiteral(integer_literal)) => integer_literal,
        _ => {
            panic!("Expected an integer literal as the right expression");
        }
    };
    assert_eq!(integer_literal.value, value);
}

#[rstest]
#[case("5 + 5;", 5, "+", 5)]
#[case("5 - 5;", 5, "-", 5)]
#[case("5 * 5;", 5, "*", 5)]
#[case("5 / 5;", 5, "/", 5)]
#[case("5 > 5;", 5, ">", 5)]
#[case("5 < 5;", 5, "<", 5)]
#[case("5 == 5;", 5, "==", 5)]
#[case("5 != 5;", 5, "!=", 5)]
fn test_infix_expression(
    #[case] input: &str,
    #[case] left_value: i64,
    #[case] operator: &str,
    #[case] right_value: i64,
) {
    use crate::ast::expression_statement::ExpressionStatement;

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    let errors = parser.errors.into_iter();

    // errors.clone().into_iter().for_each(|e| {
    //     eprintln!("Error: {} at token {:?}", e.message, e.token);
    // });

    assert_eq!(errors.len(), 0);

    let mut statements = program.statements.iter();

    let first_statement = statements.next();

    let infix_expression = match first_statement {
        Some(StatementType::Expr(Some(ExpressionType::Statement(expr)))) => {
            match expr.as_ref() {
                // deref the Box here
                ExpressionType::Infix(infix) => infix,
                _ => panic!("Expected an infix expression"),
            }
        }
        _ => panic!("Expected an expression statement"),
    };

    let infix_left_expr = match infix_expression.left.as_ref() {
        Some(left_expr) => match left_expr {
            ExpressionType::IntegerLiteral(integer_literal) => integer_literal,
            _ => {
                panic!("Expected an integer literal as the left expression");
            }
        },
        None => {
            panic!("Expected a left expression in the infix expression");
        }
    };
    let infix_operator = infix_expression.operator.clone();
    let infix_right_expr = match infix_expression.right.as_ref() {
        Some(right_expr) => match right_expr {
            ExpressionType::IntegerLiteral(integer_literal) => integer_literal,
            _ => {
                panic!("Expected an integer literal as the right expression");
            }
        },
        None => {
            panic!("Expected a right expression in the infix expression");
        }
    };

    assert_eq!(infix_left_expr.value, left_value);
    assert_eq!(infix_operator, operator);
    assert_eq!(infix_right_expr.value, right_value);
}
