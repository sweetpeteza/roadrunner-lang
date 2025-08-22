use rstest::rstest;

use crate::{ast::Node, object::Object};

#[derive(Debug)]
pub struct Evaluator {}

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn eval(&self, node: Node) -> Object {
        use crate::ast::Node::*;
        match node {
            Program { statements } => self.eval_statements(statements),
            ExprStmt { expression } => match expression {
                None => return NULL,
                Some(expr) => self.eval(*expr),
            },
            IntegerLiteral { value } => Object::Integer(value),
            BooleanLiteral { value } => self.native_bool_to_boolean_object(value),
            Prefix { operator, right } => match right {
                None => return NULL,
                Some(r) => self.eval_prefix_expression(operator, self.eval(*r)),
            },
            _ => NULL,
        }
    }

    fn eval_statements(&self, statements: Vec<Node>) -> Object {
        let mut result = NULL;
        for statement in statements {
            result = self.eval(statement);
        }

        result
    }

    fn native_bool_to_boolean_object(&self, input: bool) -> Object {
        if input {
            TRUE
        } else {
            FALSE
        }
    }

    fn eval_prefix_expression(&self, operator: String, right: Object) -> Object {
        match operator.as_str() {
            "!" => self.eval_bang_operator_expression(right),
            "-" => self.eval_minus_prefix_operator_expression(right),
            _ => NULL,
        }
    }

    fn eval_bang_operator_expression(&self, right: Object) -> Object {
        match right {
            Object::Boolean(true) => FALSE,
            Object::Boolean(false) => TRUE,
            Object::Integer(_) => FALSE,
            NULL => TRUE,
            _ => FALSE,
        }
    }

    fn eval_minus_prefix_operator_expression(&self, right: Object) -> Object {
        match right {
            Object::Integer(value) => Object::Integer(-value),
            _ => NULL,
        }
    }
}

#[cfg(test)]
use crate::lexer::Lexer;
use crate::parser::Parser;

#[rstest]
#[case(Node::IntegerLiteral { value: 5 }, Object::Integer(5))]
#[case(Node::IntegerLiteral { value: 10 }, Object::Integer(10))]
#[case(Node::BooleanLiteral { value: true }, Object::Boolean(true))]
#[case(Node::BooleanLiteral { value: false }, Object::Boolean(false))]
fn test_eval(#[case] input: Node, #[case] expected: Object) {
    let evaluator = Evaluator {};
    let evaluated = evaluator.eval(input);
    assert_eq!(evaluated, expected);
}

#[rstest]
#[case("!true", Object::Boolean(false))]
#[case("!false", Object::Boolean(true))]
#[case("!5", Object::Boolean(false))]
#[case("!!true", Object::Boolean(true))]
#[case("!!false", Object::Boolean(false))]
#[case("!!5", Object::Boolean(true))]
fn test_bang_operator(#[case] input: &str, #[case] expected: Object) {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    let evaluator = Evaluator::new();
    let evaluated = evaluator.eval(program);
    assert_eq!(evaluated, expected);
}

#[rstest]
#[case("5", Object::Integer(5))]
#[case("10", Object::Integer(10))]
#[case("-5", Object::Integer(-5))]
#[case("-10", Object::Integer(-10))]
fn test_minus_prefix_operator(#[case] input: &str, #[case] expected: Object) {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    let evaluator = Evaluator::new();
    let evaluated = evaluator.eval(program);
    assert_eq!(evaluated, expected);
}
