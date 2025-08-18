use std::fmt::Debug;

use rstest::rstest;
use tracing::{debug, error, info};
use tracing_test::traced_test;

use crate::ast::node::Node;
use crate::ast::precedence::Precedence;
use crate::{lexer::Lexer, token::Token};

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    pub errors: Vec<ParseError>,
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

    // pub fn parse_program(&mut self) -> Program {
    //     let mut program = Program::new();
    pub fn parse_program(&mut self) -> Node {
        info!("BEGIN parse_program");
        let mut statements = vec![];

        while self.current_token != Token::Eof {
            match self.parse_statement() {
                Ok(statement) => {
                    //                    program.statements.push(statement);
                    statements.push(*statement);
                }
                Err(e) => {
                    self.errors.push(e); // Collect errors
                }
            }
            self.next_token(); // Move to the next token
        }
        //        program
        Node::Program { statements }
    }

    // fn parse_statement(&mut self) -> Option<Result<StatementType, ParseError>> {
    fn parse_expression_statement(&mut self) -> Result<Box<Node>, ParseError> {
        info!("BEGIN parse_expression_statement");
        let expression = self.parse_expression(Precedence::Lowest);

        let statement = Node::ExprStmt {
            expression: expression,
        };

        if self.peek_token == Token::Semicolon {
            self.next_token(); // Consume the semicolon
        }

        info!("END parse_expression_statement");
        Ok(Box::new(statement))
    }

    // fn parse_expression(&mut self, precedence: Precedence) -> Option<ExpressionType> {
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<Node>> {
        use crate::token::Token::*;
        info!("BEGIN parse_expression with precedence: {:?}", precedence);

        // this is where the book has a hashmap of prefix functions
        let prefix = match self.current_token.clone() {
            Ident(_) => self.parse_identifier(),
            Int(_) => self.parse_integer_literal(),
            Bang | Minus => self.parse_prefix_expression(),
            True | False => self.parse_boolean_literal(),
            Lparen => self.parse_grouped_expression(),
            If => self.parse_if_expression(),
            Function => self.parse_function_literal(),
            _ => None,
        };

        if prefix.is_none() {
            return None;
        }

        let mut left_expression = prefix;

        while &self.peek_token.clone() != &Token::Semicolon
            && precedence < self.get_precedence(&self.peek_token.clone())
        {
            // this is where the book has a hashmap of infix functions
            left_expression = match self.peek_token.clone() {
                Lparen => self.parse_call_expression(left_expression),
                Plus | Minus | Slash | Asterisk | Eq | NotEq | LessThan | GreaterThan => {
                    self.next_token(); // move past the infix operator
                    self.parse_infix_expression(left_expression)
                }
                _ => {
                    info!("END parse_expression");
                    return left_expression; // No infix function, return the left expression
                }
            };
        }

        info!("END parse_expression");
        left_expression
    }

    // fn parse_infix_expression(&mut self, left: ExpressionType) -> Option<ExpressionType> {
    fn parse_infix_expression(&mut self, left: Option<Box<Node>>) -> Option<Box<Node>> {
        info!("BEGIN parse_infix_expression");
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

            return left;
        }

        info!("END parse_infix_expression");
        Some(Box::new(Node::Infix {
            left,
            operator,
            right,
        }))

        // Some(ExpressionType::Statement(Box::new(ExpressionType::Infix(
        //     InfixExpression::new(
        //         current_token,
        //         Box::new(Some(left)),
        //         operator,
        //         Box::new(right),
        //     ),
        // ))))
    }

    fn parse_call_expression(&mut self, function: Option<Box<Node>>) -> Option<Box<Node>> {
        info!("BEGIN parse_call_expression");
        let token = self.peek_token.clone();
        let arguments = self.parse_call_arguments();

        info!("END parse_call_expression");
        Some(Box::new(Node::Call {
            function,
            arguments,
        }))
    }

    fn parse_call_arguments(&mut self) -> Vec<Node> {
        info!("BEGIN parse_call_arguments");
        let mut args = Vec::new();

        if self.peek_token == Token::Rparen {
            self.next_token();

            info!("END parse_call_arguments - pt == rparen");
            return args;
        }

        self.next_token();

        if let Some(e) = self.parse_expression(Precedence::Lowest) {
            args.push(*e);
        }

        while self.peek_token == Token::Comma {
            self.next_token();
            self.next_token();
            if let Some(e) = self.parse_expression(Precedence::Lowest) {
                args.push(*e);
            }
        }

        if self.peek_token != Token::Rparen {
            info!("END parse_call_arguments - pt != rparen");
            return args;
        } else {
            self.next_token();
        }

        info!("END parse_call_arguments");
        args
    }

    fn parse_if_expression(&mut self) -> Option<Box<Node>> {
        info!("BEGIN parse_if_expression");
        // first token is if
        let if_token = self.current_token.clone();

        if self.peek_token != Token::Lparen {
            return None;
        }

        self.next_token(); // Consume the 'if' token

        let condition = self.parse_expression(Precedence::Lowest);

        // TODO combine this if check with next_token in expect_peek fn
        if self.current_token != Token::Rparen {
            return None;
        }

        self.next_token(); // Consume the closing parenthesis

        if self.current_token != Token::Lbrace {
            return None;
        }

        let consequence = self.parse_block_statement();

        let alternative = if self.peek_token == Token::Else {
            self.next_token(); // consume r brace

            match self.peek_token {
                Token::Lbrace => {
                    self.next_token(); // consume else
                    self.parse_block_statement()
                }
                _ => None,
            }
        } else {
            None
        };

        info!("END parse_if_expression");
        Some(Box::new(Node::If {
            condition,
            consequence,
            alternative,
        }))
        //     Some(ExpressionType::Statement(Box::new(ExpressionType::If(
        //         IfExpression::new(if_token, Box::new(expression), consequence, alternative),
        //     ))))
    }

    fn parse_function_literal(&mut self) -> Option<Box<Node>> {
        info!("BEGIN parse_function_literal");
        let token = self.current_token.clone();

        self.next_token();

        if self.current_token != Token::Lparen {
            info!("END parse_function_literal - did not find l paren");
            return None;
        }

        let parameters: Vec<Node> = self.parse_fn_params();

        if self.peek_token != Token::Lbrace {
            info!("END parse_function_literal - did not find l brace");
            let body = Some(Box::new(Node::Block { statements: vec![] }));
            return Some(Box::new(Node::Function {
                // token,
                parameters,
                body,
            }));
            // return Some(ExpressionType::Function(FunctionLiteral {
            //     token,
            //     parameters,
            //     body: empty_body,
            // }));
        }

        self.next_token(); // consume l brace

        let body = self.parse_block_statement();

        info!("END parse_function_literal");
        Some(Box::new(Node::Function {
            // token,
            parameters,
            body,
        }))
        // Some(ExpressionType::Function(FunctionLiteral {
        //     token,
        //     parameters,
        //     body,
        // }))
    }

    fn parse_fn_params(&mut self) -> Vec<Node> {
        info!("BEGIN parse_fn_params");

        self.next_token(); // consume l paren

        let mut params = Vec::new();

        if self.current_token == Token::Rparen {
            self.next_token(); // consume r paren
            info!("END parse_fn_params - no params");
            return params;
        }

        // let ident = Identifier::new(self.current_token.clone().to_literal());

        // params.push(ident);
        params.push(Node::Identifier {
            name: self.current_token.clone().to_literal(),
        });

        while self.peek_token == Token::Comma {
            self.next_token(); // consume comma
            self.next_token(); // consume param

            // params.push(Identifier::new(self.current_token.clone().to_literal()));
            params.push(Node::Identifier {
                name: self.current_token.clone().to_literal(),
            });
        }

        self.next_token(); // consume last param

        if self.current_token != Token::Rparen {
            info!("END parse_fn_params - no rparen found");
            return vec![];
        }
        info!("END parse_fn_params");

        params
    }

    fn parse_block_statement(&mut self) -> Option<Box<Node>> {
        info!("BEGIN parse_block_statement");
        // let mut block = BlockStatement::new(self.current_token.clone());
        let mut statements = Vec::new();

        self.next_token(); // Consume the opening brace

        while self.current_token != Token::Rbrace && self.current_token != Token::Eof {
            match self.parse_statement() {
                // Ok(stmt) => block.statements.push(stmt),
                Ok(stmt) => statements.push(*stmt),
                Err(e) => {
                    error!("Error parsing block statement: {:?}", e);
                    self.errors.push(e);
                }
            }
            self.next_token(); // Move to the next token
        }

        info!("END parse_block_statement");
        // block
        Some(Box::new(Node::Block { statements }))
    }

    fn get_precedence(&self, token: &Token) -> Precedence {
        use crate::token::Token::*;
        match token {
            Lparen => Precedence::Call,
            Eq | NotEq => Precedence::Equals,
            LessThan | GreaterThan => Precedence::LessGreater,
            Plus | Minus => Precedence::Sum,
            Asterisk | Slash => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<Node>> {
        info!("BEGIN parse_prefix_expression");
        match self.current_token {
            Token::Bang | Token::Minus => {
                let current_token = self.current_token.clone();
                let operator = current_token.to_literal();
                self.next_token();
                let right = self.parse_expression(Precedence::Prefix);
                if right.is_none() {
                    self.errors.push(ParseError {
                        message: "Expected expression after prefix operator".to_string(),
                        token: self.current_token.clone(),
                    });
                }
                info!("END parse_prefix_expression");
                Some(Box::new(Node::Prefix {
                    // current_token,
                    operator,
                    right,
                }))
                // Some(NodeType::Statement(Box::new(ExpressionType::Prefix(
                //     PrefixExpression::new(current_token, operator, Box::new(right)),
                // ))))
            }
            Token::Lbrace => {
                let current_token = self.current_token.clone();
                let operator = current_token.to_literal();
                let right = self.parse_grouped_expression();

                Some(Box::new(Node::Prefix {
                    // current_token,
                    operator,
                    right,
                }))
                // Some(ExpressionType::Statement(Box::new(ExpressionType::Prefix(
                //     PrefixExpression::new(current_token, operator, Box::new(expression)),
                // ))))
            }
            _ => {
                info!("END parse_prefix_expression");
                None
            }
        }
    }

    fn parse_grouped_expression(&mut self) -> Option<Box<Node>> {
        info!("BEGIN parse_grouped_expression");

        self.next_token(); // Consume the opening parenthesis

        let expression = self.parse_expression(Precedence::Lowest);

        if self.peek_token != Token::Rparen {
            return expression;
        }

        self.next_token(); // Consume the closing parenthesis

        info!("END parse_grouped_expression");
        expression
    }

    fn parse_identifier(&mut self) -> Option<Box<Node>> {
        info!("BEGIN parse_identifier");
        if let Token::Ident(ref ident) = self.current_token {
            info!("END parse_identifier");
            Some(Box::new(Node::Identifier {
                name: ident.clone(),
            }))
            // Some(ExpressionType::Identifier(Identifier::new(ident.clone())))
        } else {
            info!("END parse_identifier - not id");
            None
        }
    }

    fn parse_integer_literal(&mut self) -> Option<Box<Node>> {
        info!("BEGIN parse_integer_literal");
        if let Token::Int(value) = self.current_token {
            info!("END parse_integer_literal");
            Some(Box::new(Node::IntegerLiteral { value }))
            // Some(ExpressionType::IntegerLiteral(IntegerLiteral::new(
            //     self.current_token.clone(),
            //     value.clone(),
            // )))
        } else {
            info!(
                "END parse_integer_literal - not int, {:?}",
                self.current_token
            );
            None
        }
    }

    // fn parse_boolean_literal(&mut self) -> Option<NodeType> {
    fn parse_boolean_literal(&mut self) -> Option<Box<Node>> {
        info!("BEGIN parse_boolean_literal");
        match self.current_token {
            Token::True | Token::False => {
                info!("END parse_boolean_literal");
                Some(Box::new(Node::BooleanLiteral {
                    // self.current_token.clone(),
                    value: self.current_token == Token::True,
                }))
            }
            _ => {
                info!("END parse_boolean_literal");
                None
            }
        }
    }

    fn parse_statement(&mut self) -> Result<Box<Node>, ParseError> {
        info!("BEGIN parse_statement");
        debug!("Current token: {:?}", self.current_token);
        let statement = match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        };
        info!("END parse_statement");
        debug!("Parsed statement: {:?}", statement);
        statement
    }

    fn parse_let_statement(&mut self) -> Result<Box<Node>, ParseError> {
        info!("BEGIN parse_let_statement");
        let let_token = self.current_token.clone();

        self.next_token(); // Move past the 'let' token
        let name = if let Token::Ident(name) = self.current_token.clone() {
            name
        } else {
            info!("END parse_let_statement - not id");
            return Err(ParseError {
                message: "Expected identifier after 'let'".to_string(),
                token: self.current_token.clone(),
            });
        };

        if self.peek_token != Token::Assign {
            info!("END parse_let_statement - no assign");
            return Err(ParseError {
                message: "Expected '=' after variable name".to_string(),
                token: self.current_token.clone(),
            });
        }
        self.next_token(); //
        self.next_token(); // Move past the identifier

        let value = self.parse_expression(Precedence::Lowest);

        debug!(
            "current_token: {:?}, peek_token: {:?}",
            self.current_token, self.peek_token
        );

        while self.current_token != Token::Semicolon {
            self.next_token(); // Skip tokens until we reach a semicolon
        }

        info!("END parse_let_statement");
        let name = Some(Box::new(Node::Identifier { name }));
        Ok(Box::new(Node::Let {
            // let_token,
            name,
            value,
        }))
    }

    fn parse_return_statement(&mut self) -> Result<Box<Node>, ParseError> {
        info!("BEGIN parse_return_statement");
        let return_token = self.current_token.clone();

        self.next_token(); // Skip tokens until we reach a semicolon
        while self.peek_token == Token::Semicolon {
            self.next_token(); // Skip tokens until we reach a semicolon
        }
        let return_value = self.parse_expression(Precedence::Lowest);
        let return_statement = Node::Return {
            /*return_token,*/ return_value,
        };

        info!("END parse_return_statement");
        Ok(Box::new(return_statement))
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();

        debug!("ct: {:?} | pt: {:?}", self.current_token, self.peek_token);
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

#[cfg(test)]
#[derive(Debug, PartialEq)]
enum TestValue {
    Integer(i64),
    Boolean(bool),
}

#[rstest]
#[traced_test]
fn test_let_statements() {
    let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        let foobar = 1 + 2;
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

    let statements = match program {
        Node::Program { statements, .. } => statements,
        _ => panic!("Expected a program node"),
    };
    dbg!(&statements);
    assert_eq!(errors.len(), 0);
    assert_eq!(statements.len(), 4);
    assert_eq!(
        statements[0],
        Node::Let {
            // Token::Let,
            name: Some(Box::new(Node::Identifier {
                name: "x".to_string()
            })),
            value: Some(Box::new(Node::IntegerLiteral { value: 5 })),
        }
    );

    assert_eq!(
        statements[1],
        Node::Let {
            // Token::Let,
            name: Some(Box::new(Node::Identifier {
                name: "y".to_string()
            })),
            value: Some(Box::new(Node::IntegerLiteral { value: 10 })),
        }
    );
    assert_eq!(
        statements[2],
        Node::Let {
            // Token::Let,
            name: Some(Box::new(Node::Identifier {
                name: "foobar".to_string()
            })),
            value: Some(Box::new(Node::IntegerLiteral { value: 838383 })),
        }
    );
    assert_eq!(
        statements[3],
        Node::Let {
            // Token::Let,
            name: Some(Box::new(Node::Identifier {
                name: "foobar".to_string()
            })),
            value: Some(Box::new(Node::Infix {
                left: Some(Box::new(Node::IntegerLiteral { value: 1 })),
                operator: "+".to_string(),
                right: Some(Box::new(Node::IntegerLiteral { value: 2 }))
            })),
        }
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

    let mut errors = parser.errors.into_iter();
    let statements = match program {
        Node::Program { statements, .. } => statements,
        _ => panic!("Expected a program node"),
    };

    assert_eq!(statements.len(), 3);

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

// #[rstest]
// fn test_return_statements() {
//     let input = "
//         return 5;
//         return 10;
//         return 838383;
//         ";
//
//     let mut lexer = Lexer::new(input);
//     let mut parser = Parser::new(&mut lexer);
//     let program = parser.parse_program();
//
//     let errors = parser.errors.into_iter();
//
//     errors.clone().into_iter().for_each(|e| {
//         eprintln!("Error: {} at token {:?}", e.message, e.token);
//     });
//
//     assert_eq!(errors.len(), 0);
//
//     let statements = match program {
//         Node::Program { statements, .. } => statements,
//         _ => panic!("Expected a program node"),
//     };
//     assert_eq!(statements.len(), 3);
//     dbg!(&statements);
//     let first_statement = statements.first();
//     let first_statement = first_statement.expect("Expected at least one statement");
//     let expected = Node::Return {
//         return_value: Some(Box::new(Node::IntegerLiteral { value: 5 })),
//     };
//     assert_eq!(first_statement, &expected);
// }

// #[rstest]
// fn test_identifier_expression() {
//     let input = "foobar;";
//
//     let mut lexer = Lexer::new(input);
//     let mut parser = Parser::new(&mut lexer);
//     let program = parser.parse_program();
//
//     let errors = parser.errors.into_iter();
//
//     errors.clone().into_iter().for_each(|e| {
//         eprintln!("Error: {} at token {:?}", e.message, e.token);
//     });
//
//     assert_eq!(errors.len(), 0);
//     assert_eq!(program.statements.len(), 1);
//
//     if let Some(let_statement) = program.statements.first() {
//         if let StatementType::Let(let_stmt) = let_statement {
//             match &let_stmt.value {
//                 Some(ExpressionType::Identifier(ident)) => {
//                     assert_eq!(ident.value, "foobar");
//                 }
//                 _ => {
//                     panic!("Expected an expression in the let statement");
//                 }
//             }
//         }
//     }
// }
//
// #[rstest]
// fn test_integer_literal_expression() {
//     let input = "5;";
//
//     let mut lexer = Lexer::new(input);
//     let mut parser = Parser::new(&mut lexer);
//     let program = parser.parse_program();
//
//     let errors = parser.errors.into_iter();
//
//     errors.clone().into_iter().for_each(|e| {
//         eprintln!("Error: {} at token {:?}", e.message, e.token);
//     });
//
//     assert_eq!(errors.len(), 0);
//     assert_eq!(program.statements.len(), 1);
//
//     if let Some(integer_literal) = program.statements.first() {
//         if let StatementType::Expr(int_literal) = integer_literal {
//             match int_literal {
//                 Some(ExpressionType::IntegerLiteral(literal)) => {
//                     assert_eq!(literal.value, 5);
//                 }
//                 _ => panic!("Expected an integer literal expression"),
//             }
//         }
//     }
// }
//
// #[rstest]
// fn test_boolean_literal_expression() {
//     let input = "true;";
//
//     let mut lexer = Lexer::new(input);
//     let mut parser = Parser::new(&mut lexer);
//     let program = parser.parse_program();
//
//     let errors = parser.errors.into_iter();
//
//     errors.clone().into_iter().for_each(|e| {
//         eprintln!("Error: {} at token {:?}", e.message, e.token);
//     });
//
//     assert_eq!(errors.len(), 0);
//     assert_eq!(program.statements.len(), 1);
//
//     if let Some(boolean_literal) = program.statements.first() {
//         if let StatementType::Expr(bool_literal) = boolean_literal {
//             match bool_literal {
//                 Some(ExpressionType::BooleanLiteral(literal)) => {
//                     assert_eq!(literal.value, true);
//                 }
//                 _ => panic!(
//                     "Expected an boolean literal expression, got: {:?}",
//                     bool_literal
//                 ),
//             }
//         }
//     }
// }
//
// #[rstest]
// #[case("!5;", "!", TestValue::Integer(5))]
// #[case("-1;", "-", TestValue::Integer(1))]
// #[case("!true;", "!", TestValue::Boolean(true))]
// #[case("!false;", "!", TestValue::Boolean(false))]
// fn test_parsing_prefix_expression(
//     #[case] input: &str,
//     #[case] operator: &str,
//     #[case] value: TestValue,
// ) {
//     let mut lexer = Lexer::new(input);
//     let mut parser = Parser::new(&mut lexer);
//     let program = parser.parse_program();
//
//     let errors = parser.errors.into_iter();
//
//     errors.clone().into_iter().for_each(|e| {
//         eprintln!("Error: {} at token {:?}", e.message, e.token);
//     });
//
//     assert_eq!(errors.len(), 0);
//     assert_eq!(program.statements.len(), 1);
//
//     let mut statements = program.statements.iter();
//
//     let first_statement = statements.next();
//
//     let expr = match first_statement {
//         Some(StatementType::Expr(expr)) => expr,
//         _ => {
//             panic!("Expected an expression statement");
//         }
//     };
//     let expr = expr.as_ref().expect("Expected an expression statement");
//
//     let expr = match expr {
//         ExpressionType::Statement(expr_stmt) => expr_stmt.as_ref(),
//         _ => panic!("Expected an expression statement"),
//     };
//
//     let prefix_expr = match expr {
//         ExpressionType::Prefix(prefix_expr) => prefix_expr,
//         _ => {
//             panic!("Expected a prefix expression");
//         }
//     };
//
//     assert_eq!(prefix_expr.operator, operator);
//
//     let right_expr = prefix_expr.right.as_ref();
//
//     let expression_literal = match right_expr {
//         Some(ExpressionType::IntegerLiteral(integer_literal)) => {
//             TestValue::Integer(integer_literal.value)
//         }
//         Some(ExpressionType::BooleanLiteral(boolean_literal)) => {
//             TestValue::Boolean(boolean_literal.value)
//         }
//         _ => {
//             panic!("Expected an integer literal as the right expression");
//         }
//     };
//     assert_eq!(expression_literal, value);
// }
//
// #[rstest]
// #[case("5 + 5;", TestValue::Integer(5), "+", TestValue::Integer(5))]
// #[case("5 - 5;", TestValue::Integer(5), "-", TestValue::Integer(5))]
// #[case("5 * 5;", TestValue::Integer(5), "*", TestValue::Integer(5))]
// #[case("5 / 5;", TestValue::Integer(5), "/", TestValue::Integer(5))]
// #[case("5 > 5;", TestValue::Integer(5), ">", TestValue::Integer(5))]
// #[case("5 < 5;", TestValue::Integer(5), "<", TestValue::Integer(5))]
// #[case("5 == 5;", TestValue::Integer(5), "==", TestValue::Integer(5))]
// #[case("5 != 5;", TestValue::Integer(5), "!=", TestValue::Integer(5))]
// #[case(
//     "true == true;",
//     TestValue::Boolean(true),
//     "==",
//     TestValue::Boolean(true)
// )]
// #[case(
//     "true != false;",
//     TestValue::Boolean(true),
//     "!=",
//     TestValue::Boolean(false)
// )]
// #[case(
//     "false == false;",
//     TestValue::Boolean(false),
//     "==",
//     TestValue::Boolean(false)
// )]
// fn test_infix_expression(
//     #[case] input: &str,
//     #[case] left_value: TestValue,
//     #[case] operator: &str,
//     #[case] right_value: TestValue,
// ) {
//     let mut lexer = Lexer::new(input);
//     let mut parser = Parser::new(&mut lexer);
//     let program = parser.parse_program();
//
//     let errors = parser.errors.into_iter();
//
//     // errors.clone().into_iter().for_each(|e| {
//     //     eprintln!("Error: {} at token {:?}", e.message, e.token);
//     // });
//
//     assert_eq!(errors.len(), 0);
//
//     let mut statements = program.statements.iter();
//
//     let first_statement = statements.next();
//
//     let infix_expression = match first_statement {
//         Some(StatementType::Expr(Some(ExpressionType::Statement(expr)))) => {
//             match expr.as_ref() {
//                 // deref the Box here
//                 ExpressionType::Infix(infix) => infix,
//                 _ => panic!("Expected an infix expression"),
//             }
//         }
//         _ => panic!("Expected an expression statement"),
//     };
//
//     let infix_left_expr = match infix_expression.left.as_ref() {
//         Some(left_expr) => match left_expr {
//             ExpressionType::IntegerLiteral(integer_literal) => {
//                 TestValue::Integer(integer_literal.value)
//             }
//             ExpressionType::BooleanLiteral(boolean_literal) => {
//                 TestValue::Boolean(boolean_literal.value)
//             }
//             _ => {
//                 panic!("Expected an integer or boolean literal as the left expression");
//             }
//         },
//         None => {
//             panic!("Expected a left expression in the infix expression");
//         }
//     };
//     assert_eq!(infix_left_expr, left_value);
//
//     let infix_operator = infix_expression.operator.clone();
//     assert_eq!(infix_operator, operator);
//
//     let infix_right_expr = match infix_expression.right.as_ref() {
//         Some(right_expr) => match right_expr {
//             ExpressionType::IntegerLiteral(integer_literal) => {
//                 TestValue::Integer(integer_literal.value)
//             }
//             ExpressionType::BooleanLiteral(boolean_literal) => {
//                 TestValue::Boolean(boolean_literal.value)
//             }
//             _ => {
//                 panic!("Expected an integer or boolean literal as the right expression");
//             }
//         },
//         None => {
//             panic!("Expected a right expression in the infix expression");
//         }
//     };
//     assert_eq!(infix_right_expr, right_value);
// }
//
// #[traced_test]
// #[rstest]
// #[case("true", "true")]
// #[case("false", "false")]
// #[case("3 > 5 == false", "((3 > 5) == false)")]
// #[case("3 < 5 == true", "((3 < 5) == true)")]
// #[case("-a * b", "((-a) * b)")]
// #[case("!-a", "(!(-a))")]
// #[case("a + b + c", "((a + b) + c)")]
// #[case("a + b - c", "((a + b) - c)")]
// #[case("a * b * c", "((a * b) * c)")]
// #[case("a * b / c", "((a * b) / c)")]
// #[case("a + b / c", "(a + (b / c))")]
// #[case("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)")]
// #[case("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)")]
// #[case("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))")]
// #[case("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))")]
// #[case("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))")]
// #[case("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)")]
// #[case("(5 + 5) * 2", "((5 + 5) * 2)")]
// #[case("2 / (5 + 5)", "(2 / (5 + 5))")]
// #[case("-(5 + 5)", "(-(5 + 5))")]
// #[case("!(true == true)", "(!(true == true))")]
// #[case("(a + add(b * c)) + d", "((a + add((b * c))) + d)")]
// #[case(
//     "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
//     "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"
// )]
// #[case("add(a + b + c * d / f + g)", "add((((a + b) + ((c * d) / f)) + g))")]
// fn test_operator_precedence_parsing(#[case] input: &str, #[case] expected_output: &str) {
//     use crate::ast::traits::Node;
//
//     let mut lexer = Lexer::new(input);
//     let mut parser = Parser::new(&mut lexer);
//     let program = parser.parse_program();
//
//     let errors = parser.errors.into_iter();
//
//     errors.clone().into_iter().for_each(|e| {
//         eprintln!("Error: {} at token {:?}", e.message, e.token);
//     });
//
//     assert_eq!(errors.len(), 0);
//
//     // dbg!(&program);
//
//     assert_eq!(program.string(), expected_output);
// }
//
// #[traced_test]
// #[rstest]
// fn test_if_expresssion() {
//     let input = "if (x < y) { x }";
//
//     let mut lexer = Lexer::new(input);
//     let mut parser = Parser::new(&mut lexer);
//     let program = parser.parse_program();
//
//     let errors = parser.errors.into_iter();
//
//     // errors.clone().into_iter().for_each(|e| {
//     //     eprintln!("Error: {} at token {:?}", e.message, e.token);
//     // });
//
//     assert_eq!(errors.len(), 0);
//
//     let mut statements = program.statements.iter();
//     let first_statement = statements.next();
//
//     dbg!(&first_statement);
//
//     let if_expression = match first_statement {
//         Some(StatementType::Expr(Some(ExpressionType::Statement(expr)))) => {
//             match expr.as_ref() {
//                 // deref the Box here
//                 ExpressionType::If(if_expr) => if_expr,
//                 _ => panic!("Expected an if expression, got {:?}", expr),
//             }
//         }
//         _ => panic!(
//             "Expected an expression statement, got {:?}",
//             first_statement
//         ),
//     };
//
//     let _infix_expr = match if_expression.condition.as_ref() {
//         Some(ExpressionType::Statement(expr)) => match expr.as_ref() {
//             ExpressionType::Infix(infix) => {
//                 match infix.left.as_ref() {
//                     Some(ExpressionType::Identifier(ident)) => {
//                         assert_eq!(ident.value, "x");
//                     }
//                     _ => panic!("Expected an identifier as the left expression"),
//                 }
//                 match infix.right.as_ref() {
//                     Some(ExpressionType::Identifier(ident)) => {
//                         assert_eq!(ident.value, "y");
//                     }
//                     _ => panic!("Expected an identifier as the right expression"),
//                 }
//                 assert_eq!(infix.operator, "<");
//                 infix
//             }
//             _ => panic!(
//                 "Expected an infix expression as the condition, got {:?}",
//                 expr
//             ),
//         },
//
//         _ => panic!(
//             "Expected an infix expression as the condition, got {:?}",
//             if_expression.condition
//         ),
//     };
//
//     let block_statement = &if_expression.consequence;
//
//     let statements = &block_statement.statements;
//     assert_eq!(statements.len(), 1);
//
//     let first_statement = statements
//         .first()
//         .expect("Expected a statement in the block");
//
//     match first_statement {
//         StatementType::Expr(Some(ExpressionType::Identifier(ident))) => {
//             assert_eq!(ident.value, "x");
//         }
//         _ => panic!("Expected an identifier expression in the block statement"),
//     }
// }
//
// #[traced_test]
// #[rstest]
// fn test_if_else_expresssion() {
//     let input = "if (x < y) { x } else { y }";
//
//     let mut lexer = Lexer::new(input);
//     let mut parser = Parser::new(&mut lexer);
//     let program = parser.parse_program();
//
//     let errors = parser.errors.into_iter();
//
//     // errors.clone().into_iter().for_each(|e| {
//     //     eprintln!("Error: {} at token {:?}", e.message, e.token);
//     // });
//
//     assert_eq!(errors.len(), 0);
//
//     let mut statements = program.statements.iter();
//     let first_statement = statements.next();
//
//     dbg!(&first_statement);
//
//     let if_expression = match first_statement {
//         Some(StatementType::Expr(Some(ExpressionType::Statement(expr)))) => {
//             match expr.as_ref() {
//                 // deref the Box here
//                 ExpressionType::If(if_expr) => if_expr,
//                 _ => panic!("Expected an if expression, got {:?}", expr),
//             }
//         }
//         _ => panic!(
//             "Expected an expression statement, got {:?}",
//             first_statement
//         ),
//     };
//
//     let _infix_expr = match if_expression.condition.as_ref() {
//         Some(ExpressionType::Statement(expr)) => match expr.as_ref() {
//             ExpressionType::Infix(infix) => {
//                 match infix.left.as_ref() {
//                     Some(ExpressionType::Identifier(ident)) => {
//                         assert_eq!(ident.value, "x");
//                     }
//                     _ => panic!("Expected an identifier as the left expression"),
//                 }
//                 match infix.right.as_ref() {
//                     Some(ExpressionType::Identifier(ident)) => {
//                         assert_eq!(ident.value, "y");
//                     }
//                     _ => panic!("Expected an identifier as the right expression"),
//                 }
//                 assert_eq!(infix.operator, "<");
//                 infix
//             }
//             _ => panic!(
//                 "Expected an infix expression as the condition, got {:?}",
//                 expr
//             ),
//         },
//
//         _ => panic!(
//             "Expected an infix expression as the condition, got {:?}",
//             if_expression.condition
//         ),
//     };
//
//     let block_statement = &if_expression.consequence;
//
//     let statements = &block_statement.statements;
//     assert_eq!(statements.len(), 1);
//
//     let first_statement = statements
//         .first()
//         .expect("Expected a statement in the block");
//
//     match first_statement {
//         StatementType::Expr(Some(ExpressionType::Identifier(ident))) => {
//             assert_eq!(ident.value, "x");
//         }
//         _ => panic!("Expected an identifier expression in the block statement"),
//     }
//
//     let alternative_block = if_expression
//         .alternative
//         .as_ref()
//         .expect("Expected an alternative block");
//
//     let alternative_statements = &alternative_block.statements;
//
//     assert_eq!(alternative_statements.len(), 1);
//
//     let alternative_first_statement = alternative_statements
//         .first()
//         .expect("Expected a statement in the alternative block");
//
//     match alternative_first_statement {
//         StatementType::Expr(Some(ExpressionType::Identifier(ident))) => {
//             assert_eq!(ident.value, "y");
//         }
//         _ => panic!("Expected an identifier expression in the alternative block statement"),
//     }
// }
//
// #[traced_test]
// #[rstest]
// #[case("fn(x,y) { x + y; }", vec!["x","y"])]
// #[case("fn() {}", vec![])]
// #[case("fn(x) {}", vec!["x"])]
// #[case("fn(x,y,z) {}", vec!["x","y", "z"])]
// fn test_parse_function_literal(#[case] input: &str, #[case] expected_params: Vec<&str>) {
//     use crate::ast::traits::Node;
//
//     let mut lexer = Lexer::new(input);
//     let mut parser = Parser::new(&mut lexer);
//     let program = parser.parse_program();
//
//     let errors = parser.errors.into_iter();
//
//     // errors.clone().into_iter().for_each(|e| {
//     //     eprintln!("Error: {} at token {:?}", e.message, e.token);
//     // });
//
//     assert_eq!(errors.len(), 0);
//
//     let mut statements = program.statements.iter();
//     let first_statement = statements.next();
//
//     dbg!(&first_statement);
//
//     let function_literal = match first_statement {
//         Some(StatementType::Expr(Some(ExpressionType::Function(fn_literal)))) => fn_literal,
//         _ => panic!("Expected an function literal, got {:?}", first_statement),
//     };
//
//     let params = function_literal.parameters.iter();
//
//     assert_eq!(params.len(), expected_params.len());
//
//     let expected_params = expected_params.join(", ");
//     let params_joined = function_literal
//         .parameters
//         .clone()
//         .into_iter()
//         .map(|p| p.token_literal())
//         .collect::<Vec<String>>()
//         .join(", ");
//
//     assert_eq!(params_joined, expected_params);
//
//     let mut statements = function_literal.body.statements.iter();
//
//     let body_statement = statements.next();
//
//     dbg!(&body_statement);
//
//     match body_statement {
//         Some(StatementType::Expr(Some(ExpressionType::Statement(expr)))) => match expr.as_ref() {
//             ExpressionType::Infix(infix) => {
//                 match infix.left.as_ref() {
//                     Some(ExpressionType::Identifier(ident)) => {
//                         assert_eq!(ident.value, "x");
//                     }
//                     _ => panic!("Expected an identifier as the left expression"),
//                 }
//                 match infix.right.as_ref() {
//                     Some(ExpressionType::Identifier(ident)) => {
//                         assert_eq!(ident.value, "y");
//                     }
//                     _ => panic!("Expected an identifier as the right expression"),
//                 }
//                 assert_eq!(infix.operator, "+");
//             }
//             _ => {}
//         },
//         _ => {}
//     }
// }
//
// #[traced_test]
// #[rstest]
// fn test_call_expresssion() {
//     let input = "add(1, 2 * 3, 4 + 5)";
//
//     let mut lexer = Lexer::new(input);
//     let mut parser = Parser::new(&mut lexer);
//     let program = parser.parse_program();
//
//     let errors = parser.errors.into_iter();
//
//     // errors.clone().into_iter().for_each(|e| {
//     //     eprintln!("Error: {} at token {:?}", e.message, e.token);
//     // });
//
//     assert_eq!(errors.len(), 0);
//
//     let mut statements = program.statements.iter();
//     let first_statement = statements.next();
//
//     dbg!(&first_statement);
//
//     let call_expression = match first_statement {
//         Some(StatementType::Expr(Some(ExpressionType::Statement(expr)))) => {
//             match expr.as_ref() {
//                 // deref the Box here
//                 ExpressionType::Call(call_expr) => call_expr,
//                 _ => panic!("Expected an call expression, got {:?}", expr),
//             }
//         }
//         _ => panic!(
//             "Expected an expression statement, got {:?}",
//             first_statement
//         ),
//     };
//
//     assert_eq!(call_expression.token.to_literal(), "(");
//     assert_eq!(call_expression.arguments.len(), 3);
//
//     let function_expr = call_expression.function.as_ref();
//
//     match function_expr {
//         ExpressionType::Identifier(ident) => {
//             assert_eq!(&ident.value, "add");
//         }
//         _ => panic!("Expected identifier, got {:?}", function_expr),
//     };
// }
