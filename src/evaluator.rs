use rstest::rstest;

use crate::{ast::Node, object::Object};

#[derive(Debug)]
pub struct Evaluator {}

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn eval(&self, node: Node) -> Object {
        use crate::ast::Node::*;
        match node {
            Program { statements: _ } => self.eval_program(node),
            ExprStmt { expression } => match expression {
                None => NULL,
                Some(expr) => self.eval(*expr),
            },
            IntegerLiteral { value } => Object::Integer(value),
            BooleanLiteral { value } => self.native_bool_to_boolean_object(value),
            Prefix { operator, right } => match right {
                None => NULL,
                Some(r) => {
                    let right = self.eval(*r);
                    if right.is_error() {
                        return right;
                    }
                    self.eval_prefix_expression(operator, right)
                }
            },
            Infix {
                left,
                operator,
                right,
            } => match (left, right) {
                (Some(l), Some(r)) => {
                    let left = self.eval(*l);

                    if left.is_error() {
                        return left;
                    }

                    let right = self.eval(*r);

                    if right.is_error() {
                        return right;
                    }

                    self.eval_infix_expression(operator, left, right)
                }
                _ => NULL,
            },
            Block { statements: _ } => self.eval_block_statement(node),
            If {
                condition,
                consequence,
                alternative,
            } => {
                let condition = match condition {
                    Some(cond) => self.eval(*cond),
                    None => NULL,
                };

                if condition.is_error() {
                    return condition;
                }

                if self.is_truthy(&condition) {
                    return match consequence {
                        Some(cons) => self.eval(*cons),
                        None => NULL,
                    };
                } else if let Some(alt) = alternative {
                    return self.eval(*alt);
                } else {
                    return NULL;
                }
            }
            Return { return_value } => match return_value {
                Some(value) => {
                    let val = self.eval(*value);
                    if val.is_error() {
                        return val;
                    }
                    Object::ReturnValue(Box::new(val))
                }
                None => NULL,
            },
            _ => NULL,
        }
    }

    fn eval_program(&self, program: Node) -> Object {
        let statements = match program {
            Node::Program { statements } => statements,
            // prevent incorrect node type
            _ => return NULL,
        };

        let mut result = NULL;
        for statement in statements {
            result = self.eval(statement);

            match result {
                Object::ReturnValue(ret_val) => return *ret_val,
                Object::Error(_) => return result,
                _ => { /* continue evaluating */ }
            }
        }

        result
    }

    fn eval_block_statement(&self, block: Node) -> Object {
        let statements = match block {
            Node::Block { statements } => statements,
            // prevent incorrect node type
            _ => return NULL,
        };

        let mut result = NULL;
        for statement in statements {
            result = self.eval(statement);

            match result {
                Object::ReturnValue(ref return_val) => match return_val.as_ref() {
                    Object::Null => {}
                    _ => return result,
                },
                Object::Error(_) => return result,
                _ => { /* continue evaluating */ }
            }
        }

        result
    }

    fn is_truthy(&self, obj: &Object) -> bool {
        match obj {
            &TRUE => true,
            &FALSE => false,
            &NULL => false,
            _ => true,
        }
    }

    fn eval_infix_expression(&self, operator: String, left: Object, right: Object) -> Object {
        match (left.clone(), right.clone()) {
            (Object::Integer(left_val), Object::Integer(right_val)) => {
                self.eval_integer_infix_expression(operator, left_val, right_val)
            }
            (Object::Boolean(left_val), Object::Boolean(right_val)) => {
                self.eval_boolean_infix_expression(operator, left_val, right_val)
            }
            (left, right) if left.type_name() != right.type_name() => Object::Error(format!(
                "type mismatch: {} {} {}",
                left.type_name(),
                operator,
                right.type_name()
            )),
            _ => Object::Error(format!(
                "unknown operator: {} {} {}",
                left.type_name(),
                operator,
                right.type_name()
            )),
        }
    }

    fn eval_integer_infix_expression(&self, operator: String, left: i64, right: i64) -> Object {
        match operator.as_str() {
            "+" => Object::Integer(left + right),
            "-" => Object::Integer(left - right),
            "*" => Object::Integer(left * right),
            "/" => Object::Integer(left / right),
            "<" => self.native_bool_to_boolean_object(left < right),
            ">" => self.native_bool_to_boolean_object(left > right),
            "==" => self.native_bool_to_boolean_object(left == right),
            "!=" => self.native_bool_to_boolean_object(left != right),
            _ => Object::Error(format!("unknown operator: {} {} {}", left, operator, right)),
        }
    }

    fn eval_boolean_infix_expression(&self, operator: String, left: bool, right: bool) -> Object {
        match operator.as_str() {
            "==" => self.native_bool_to_boolean_object(left == right),
            "!=" => self.native_bool_to_boolean_object(left != right),
            _ => Object::Error(format!("unknown operator: BOOLEAN {} BOOLEAN", operator,)),
        }
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
            _ => Object::Error(format!(
                "unknown operator: {}{}",
                operator,
                right.type_name()
            )),
        }
    }

    fn eval_bang_operator_expression(&self, right: Object) -> Object {
        match right {
            TRUE => FALSE,
            FALSE => TRUE,
            Object::Integer(_) => FALSE,
            NULL => TRUE,
            _ => FALSE,
        }
    }

    fn eval_minus_prefix_operator_expression(&self, right: Object) -> Object {
        match right {
            Object::Integer(value) => Object::Integer(-value),
            _ => Object::Error(format!("unknown operator: -{}", right.type_name())),
        }
    }
}

#[cfg(test)]
use crate::{lexer::Lexer, parser::Parser};

#[rstest]
#[case(Node::IntegerLiteral { value: 5 }, Object::Integer(5))]
#[case(Node::IntegerLiteral { value: 10 }, Object::Integer(10))]
#[case(Node::BooleanLiteral { value: true }, TRUE)]
#[case(Node::BooleanLiteral { value: false }, FALSE)]
fn test_eval(#[case] input: Node, #[case] expected: Object) {
    let evaluator = Evaluator {};
    let evaluated = evaluator.eval(input);
    assert_eq!(evaluated, expected);
}

#[rstest]
#[case("!true", FALSE)]
#[case("!false", TRUE)]
#[case("!5", FALSE)]
#[case("!!true", TRUE)]
#[case("!!false", FALSE)]
#[case("!!5", TRUE)]
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

#[rstest]
#[case("5", 5)]
#[case("10", 10)]
#[case("-5", -5)]
#[case("-10", -10)]
#[case("5 + 5 + 5 + 5 - 10", 10)]
#[case("2 * 2 * 2 * 2 * 2", 32)]
#[case("-50 + 100 + -50", 0)]
#[case("5 * 2 + 10", 20)]
#[case("5 + 2 * 10", 25)]
#[case("20 + 2 * -10", 0)]
#[case("50 / 2 * 2 + 10", 60)]
#[case("2 * (5 + 10)", 30)]
#[case("3 * 3 * 3 + 10", 37)]
#[case("3 * (3 * 3) + 10", 37)]
#[case("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50)]
fn test_integer_expressions(#[case] input: &str, #[case] expected: i64) {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    let evaluator = Evaluator::new();
    let evaluated = evaluator.eval(program);
    match evaluated {
        Object::Integer(value) => assert_eq!(value, expected),
        _ => panic!("object is not Integer. got={}", evaluated),
    }
}

#[rstest]
#[case("true", TRUE)]
#[case("false", FALSE)]
#[case("1 < 2", TRUE)]
#[case("1 > 2", FALSE)]
#[case("1 < 1", FALSE)]
#[case("1 > 1", FALSE)]
#[case("1 == 1", TRUE)]
#[case("1 != 1", FALSE)]
#[case("1 == 2", FALSE)]
#[case("1 != 2", TRUE)]
fn test_boolean_expressions(#[case] input: &str, #[case] expected: Object) {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    let evaluator = Evaluator::new();
    let evaluated = evaluator.eval(program);
    assert_eq!(evaluated, expected);
}

#[rstest]
#[case("true == true", TRUE)]
#[case("false == false", TRUE)]
#[case("true == false", FALSE)]
#[case("true != false", TRUE)]
#[case("false != true", TRUE)]
#[case("(1 < 2) == true", TRUE)]
#[case("(1 < 2) == false", FALSE)]
#[case("(1 > 2) == true", FALSE)]
#[case("(1 > 2) == false", TRUE)]
fn test_boolean_infix_expressions(#[case] input: &str, #[case] expected: Object) {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    let evaluator = Evaluator::new();
    let evaluated = evaluator.eval(program);
    assert_eq!(evaluated, expected);
}

#[rstest]
#[case("if (true) { 10 }", Object::Integer(10))]
#[case("if (false) { 10 }", NULL)]
#[case("if (1) { 10 }", Object::Integer(10))]
#[case("if (1 < 2) { 10 }", Object::Integer(10))]
#[case("if (1 > 2) { 10 }", NULL)]
#[case("if (1 > 2) { 10 } else { 20 }", Object::Integer(20))]
#[case("if (1 < 2) { 10 } else { 20 }", Object::Integer(10))]
fn test_if_expressions(#[case] input: &str, #[case] expected: Object) {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    let evaluator = Evaluator::new();
    let evaluated = evaluator.eval(program);
    assert_eq!(evaluated, expected);
}

#[rstest]
#[case("return 10;", Object::Integer(10))]
#[case("return 10; 9;", Object::Integer(10))]
#[case("return 2 * 5; 9;", Object::Integer(10))]
#[case("9; return 2 * 5; 9;", Object::Integer(10))]
#[case(
    "if (10 > 1) { if (10 > 1) { return 10; } return 1; }",
    Object::Integer(10)
)]
fn test_return_statements(#[case] input: &str, #[case] expected: Object) {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    let evaluator = Evaluator::new();
    let evaluated = evaluator.eval(program);
    assert_eq!(evaluated, expected);
}

#[rstest]
#[case("5 + true;", "type mismatch: INTEGER + BOOLEAN")]
#[case("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN")]
#[case("-true", "unknown operator: -BOOLEAN")]
#[case("true + false;", "unknown operator: BOOLEAN + BOOLEAN")]
#[case("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN")]
#[case("if (10 > 1) { true + false; }", "unknown operator: BOOLEAN + BOOLEAN")]
#[case(
    "if (10 > 1) { if (10 > 1) { return true + false; } return 1; }",
    "unknown operator: BOOLEAN + BOOLEAN"
)]
fn test_error_handling(#[case] input: &str, #[case] expected_message: &str) {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    let evaluator = Evaluator::new();
    let evaluated = evaluator.eval(program);

    match evaluated {
        Object::Error(message) => assert_eq!(message, expected_message),
        _ => panic!("no error object returned. got={}", evaluated),
    }
}
