use rstest::rstest;
use tracing::{debug, info};

use crate::{
    ast::Node,
    object::{Environment, Object},
};

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

    pub fn eval(&self, node: Node, env: &mut Environment) -> Object {
        use crate::ast::Node::*;
        match node {
            Program { statements: _ } => self.eval_program(node, env),
            ExprStmt { expression } => match expression {
                None => NULL,
                Some(expr) => self.eval(*expr, env),
            },
            IntegerLiteral { value } => Object::Integer(value),
            BooleanLiteral { value } => self.native_bool_to_boolean_object(value),
            Prefix { operator, right } => match right {
                None => NULL,
                Some(r) => {
                    let right = self.eval(*r, env);
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
                    let left = self.eval(*l, env);

                    if left.is_error() {
                        return left;
                    }

                    let right = self.eval(*r, env);

                    if right.is_error() {
                        return right;
                    }

                    self.eval_infix_expression(operator, left, right)
                }
                _ => NULL,
            },
            Block { statements: _ } => self.eval_block_statement(node, env),
            If {
                condition,
                consequence,
                alternative,
            } => {
                let condition = match condition {
                    Some(cond) => self.eval(*cond, env),
                    None => NULL,
                };

                if condition.is_error() {
                    return condition;
                }

                if self.is_truthy(&condition) {
                    return match consequence {
                        Some(cons) => self.eval(*cons, env),
                        None => NULL,
                    };
                } else if let Some(alt) = alternative {
                    return self.eval(*alt, env);
                } else {
                    return NULL;
                }
            }
            Return { return_value } => match return_value {
                Some(value) => {
                    let val = self.eval(*value, env);
                    if val.is_error() {
                        return val;
                    }
                    Object::ReturnValue(Box::new(val))
                }
                None => NULL,
            },
            Let { name, value } => {
                let value_object = match value {
                    Some(val) => {
                        let obj = self.eval(*val, env);
                        if obj.is_error() {
                            return obj;
                        }
                        obj
                    }
                    None => NULL,
                };

                let name_node = match name {
                    Some(n) => n,
                    None => return NULL,
                };

                let name_str = match *name_node {
                    Node::Identifier { name } => name,
                    // prevent incorrect node type
                    _ => {
                        return Object::Error(
                            "let statement name must be an identifier".to_string(),
                        )
                    }
                };

                env.set(name_str, value_object.clone());

                value_object
            }
            Identifier { name } => match env.get(&name) {
                Some(val) => val,
                None => Object::Error(format!("identifier not found: {}", name)),
            },
            Function { parameters, body } => Object::Function {
                parameters,
                body: body,
                env: Box::new(env.clone()),
            },
            Call {
                function,
                arguments,
            } => {
                let function = match function {
                    Some(func) => self.eval(*func, env),
                    None => return NULL,
                };

                if function.is_error() {
                    return function;
                }

                info!("Function to be called: {:?}", function);
                info!("Arguments to be evaluated: {:?}", arguments);
                info!("Current environment: {:?}", env);
                let args = self.eval_expressions(arguments, env);

                if args.len() == 1 && args[0].is_error() {
                    return args[0].clone();
                }

                self.apply_function(function, args)
            }
        }
    }

    fn eval_program(&self, program: Node, environment: &mut Environment) -> Object {
        let statements = match program {
            Node::Program { statements } => statements,
            // prevent incorrect node type
            _ => return NULL,
        };

        let mut result = NULL;
        for statement in statements {
            result = self.eval(statement, environment);

            match result {
                Object::ReturnValue(ret_val) => return *ret_val,
                Object::Error(_) => return result,
                _ => { /* continue evaluating */ }
            }
        }

        result
    }

    fn apply_function(&self, function: Object, args: Vec<Object>) -> Object {
        match &function {
            Object::Function { body, .. } => {
                let mut extended_env = match self.extend_function_env(function.clone(), args) {
                    Ok(env) => env,
                    Err(err) => return err,
                };

                let evaluated = match body {
                    Some(bdy) => self.eval(*bdy.clone(), &mut extended_env),
                    None => NULL,
                };

                self.unwrap_return_value(evaluated)
            }
            _ => Object::Error(format!("not a function: {}", &function.type_name())),
        }
    }

    fn unwrap_return_value(&self, obj: Object) -> Object {
        match obj {
            Object::ReturnValue(value) => *value,
            _ => obj,
        }
    }

    fn extend_function_env(
        &self,
        function: Object,
        args: Vec<Object>,
    ) -> Result<Environment, Object> {
        match function {
            Object::Function {
                parameters,
                env: func_env,
                ..
            } => {
                let mut extended_env = Environment::new(Some(func_env));

                for (param, arg) in parameters.iter().zip(args.into_iter()) {
                    let param_name = match param {
                        Node::Identifier { name } => name.clone(),
                        // prevent incorrect node type
                        _ => {
                            return Err(Object::Error(
                                "function parameter must be an identifier".to_string(),
                            ))
                        }
                    };
                    extended_env.set(param_name, arg);
                }

                debug!("Extended function environment: {:?}", extended_env);
                Ok(extended_env)
            }
            _ => Err(Object::Error(format!(
                "not a function: {}",
                function.type_name()
            ))),
        }
    }

    fn eval_expressions(
        &self,
        expressions: Vec<Node>,
        environment: &mut Environment,
    ) -> Vec<Object> {
        let mut result = Vec::new();

        for expr in expressions {
            let evaluated = self.eval(expr, environment);
            if evaluated.is_error() {
                return vec![evaluated]; // return the error in the vector
            }
            result.push(evaluated);
        }

        debug!("Evaluated expressions: {:?}", result);
        result
    }

    fn eval_block_statement(&self, block: Node, environment: &mut Environment) -> Object {
        let statements = match block {
            Node::Block { statements } => statements,
            // prevent incorrect node type
            _ => return NULL,
        };

        let mut result = NULL;
        for statement in statements {
            result = self.eval(statement, environment);

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

#[cfg(test)]
use tracing_test::traced_test;

#[rstest]
#[case(Node::IntegerLiteral { value: 5 }, Object::Integer(5))]
#[case(Node::IntegerLiteral { value: 10 }, Object::Integer(10))]
#[case(Node::BooleanLiteral { value: true }, TRUE)]
#[case(Node::BooleanLiteral { value: false }, FALSE)]
fn test_eval(#[case] input: Node, #[case] expected: Object) {
    let evaluator = Evaluator {};
    let mut env = Environment::new(None);
    let evaluated = evaluator.eval(input, &mut env);
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
    let mut env = Environment::new(None);
    let evaluated = evaluator.eval(program, &mut env);
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
    let mut env = Environment::new(None);
    let evaluated = evaluator.eval(program, &mut env);
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
    let mut env = Environment::new(None);
    let evaluated = evaluator.eval(program, &mut env);
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
    let mut env = Environment::new(None);
    let evaluated = evaluator.eval(program, &mut env);
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
    let mut env = Environment::new(None);
    let evaluated = evaluator.eval(program, &mut env);
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
    let mut env = Environment::new(None);
    let evaluated = evaluator.eval(program, &mut env);
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
    let mut env = Environment::new(None);
    let evaluated = evaluator.eval(program, &mut env);
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
#[case("foobar", "identifier not found: foobar")]
fn test_error_handling(#[case] input: &str, #[case] expected_message: &str) {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    let evaluator = Evaluator::new();
    let mut env = Environment::new(None);
    let evaluated = evaluator.eval(program, &mut env);

    match evaluated {
        Object::Error(message) => assert_eq!(message, expected_message),
        _ => panic!("no error object returned. got={}", evaluated),
    }
}

#[rstest]
#[traced_test]
#[case("let a = 5; a;", Object::Integer(5))]
#[case("let a = 5 * 5; a;", Object::Integer(25))]
#[case("let a = 5; let b = a; b;", Object::Integer(5))]
#[case("let a = 5; let b = a; let c = a + b + 5; c;", Object::Integer(15))]
fn test_let_statements(#[case] input: &str, #[case] expected: Object) {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    let evaluator = Evaluator::new();
    let mut env = Environment::new(None);
    let evaluated = evaluator.eval(program, &mut env);
    assert_eq!(evaluated, expected);
}

#[rstest]
#[case("let identity = fn(x) { x; }; identity(5);", Object::Integer(5))]
#[case("let identity = fn(x) { return x; }; identity(5);", Object::Integer(5))]
#[case("let double = fn(x) { x * 2; }; double(5);", Object::Integer(10))]
#[case("let add = fn(x, y) { x + y; }; add(5, 5);", Object::Integer(10))]
#[case(
    "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
    Object::Integer(20)
)]
#[case("fn(x) { x; }(5)", Object::Integer(5))]
fn test_function_application(#[case] input: &str, #[case] expected: Object) {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    let evaluator = Evaluator::new();
    let mut env = Environment::new(None);
    let evaluated = evaluator.eval(program, &mut env);
    assert_eq!(evaluated, expected);
}

#[rstest]
#[traced_test]
#[case(
    "let newAdder = fn(x) { fn(y) { x + y }; }; let addTwo = newAdder(2); addTwo(2);",
    Object::Integer(4)
)]
#[case("let counter = fn(x) {   if (x > 100) {     return true;   } else {     let foobar = 9999;     counter(x + 1);   } }; counter(0);", TRUE)]
fn test_closures(#[case] input: &str, #[case] expected: Object) {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    let evaluator = Evaluator::new();
    let mut env = Environment::new(None);
    debug!("Evaluating program: {:?}", program);
    let evaluated = evaluator.eval(program, &mut env);
    assert_eq!(evaluated, expected);
}
