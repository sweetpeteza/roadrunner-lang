use std::rc::Rc;

use rstest::rstest;
use tracing::debug;

use crate::{
    ast::Node,
    environment::{Env, Environment},
    object::Object,
};

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;

/// Main evaluation function - evaluates an AST node in the given environment
pub fn eval(node: Node, env: Env) -> Object {
    use crate::ast::Node::*;
    match node {
        Program { statements } => eval_program(statements, env),
        ExprStmt { expression } => expression
            .map(|expr| eval(*expr, env))
            .unwrap_or(NULL),
        IntegerLiteral { value } => Object::Integer(value),
        BooleanLiteral { value } => native_bool_to_boolean_object(value),
        Prefix { operator, right } => eval_prefix(operator, right, env),
        Infix { left, operator, right } => eval_infix(left, operator, right, env),
        Block { statements } => eval_block(statements, env),
        If { condition, consequence, alternative } => {
            eval_if(condition, consequence, alternative, env)
        }
        Return { return_value } => eval_return(return_value, env),
        Let { name, value } => eval_let(name, value, env),
        Identifier { name } => eval_identifier(name, env),
        Function { parameters, body } => Object::Function {
            parameters,
            body,
            env: Rc::clone(&env),
        },
        Call { function, arguments } => eval_call(function, arguments, env),
    }
}

fn eval_program(statements: Vec<Node>, env: Env) -> Object {
    statements.into_iter().try_fold(NULL, |_, stmt| {
        let result = eval(stmt, Rc::clone(&env));
        match &result {
            Object::ReturnValue(val) => Err(*val.clone()),
            Object::Error(_) => Err(result),
            _ => Ok(result),
        }
    }).unwrap_or_else(|early_return| early_return)
}

fn eval_prefix(operator: String, right: Option<Box<Node>>, env: Env) -> Object {
    let right_val = right
        .map(|r| eval(*r, env))
        .unwrap_or(NULL);
    
    if right_val.is_error() {
        return right_val;
    }
    
    match operator.as_str() {
        "!" => eval_bang(right_val),
        "-" => eval_minus_prefix(right_val),
        _ => Object::Error(format!("unknown operator: {}{}", operator, right_val.type_name())),
    }
}

fn eval_bang(right: Object) -> Object {
    match right {
        TRUE => FALSE,
        FALSE => TRUE,
        NULL => TRUE,
        Object::Integer(_) => FALSE,
        _ => FALSE,
    }
}

fn eval_minus_prefix(right: Object) -> Object {
    match right {
        Object::Integer(value) => Object::Integer(-value),
        _ => Object::Error(format!("unknown operator: -{}", right.type_name())),
    }
}

fn eval_infix(
    left: Option<Box<Node>>,
    operator: String,
    right: Option<Box<Node>>,
    env: Env,
) -> Object {
    let left_val = left.map(|l| eval(*l, Rc::clone(&env))).unwrap_or(NULL);
    if left_val.is_error() {
        return left_val;
    }
    
    let right_val = right.map(|r| eval(*r, env)).unwrap_or(NULL);
    if right_val.is_error() {
        return right_val;
    }
    
    match (&left_val, &right_val) {
        (Object::Integer(l), Object::Integer(r)) => eval_integer_infix(operator, *l, *r),
        (Object::Boolean(l), Object::Boolean(r)) => eval_boolean_infix(operator, *l, *r),
        (l, r) if l.type_name() != r.type_name() => {
            Object::Error(format!("type mismatch: {} {} {}", l.type_name(), operator, r.type_name()))
        }
        _ => Object::Error(format!(
            "unknown operator: {} {} {}",
            left_val.type_name(), operator, right_val.type_name()
        )),
    }
}

fn eval_integer_infix(operator: String, left: i64, right: i64) -> Object {
    match operator.as_str() {
        "+" => Object::Integer(left + right),
        "-" => Object::Integer(left - right),
        "*" => Object::Integer(left * right),
        "/" => Object::Integer(left / right),
        "<" => native_bool_to_boolean_object(left < right),
        ">" => native_bool_to_boolean_object(left > right),
        "==" => native_bool_to_boolean_object(left == right),
        "!=" => native_bool_to_boolean_object(left != right),
        _ => Object::Error(format!("unknown operator: {} {} {}", left, operator, right)),
    }
}

fn eval_boolean_infix(operator: String, left: bool, right: bool) -> Object {
    match operator.as_str() {
        "==" => native_bool_to_boolean_object(left == right),
        "!=" => native_bool_to_boolean_object(left != right),
        _ => Object::Error(format!("unknown operator: BOOLEAN {} BOOLEAN", operator)),
    }
}

fn eval_block(statements: Vec<Node>, env: Env) -> Object {
    statements.into_iter().try_fold(NULL, |_, stmt| {
        let result = eval(stmt, Rc::clone(&env));
        match &result {
            Object::ReturnValue(val) if !matches!(val.as_ref(), Object::Null) => Err(result),
            Object::Error(_) => Err(result),
            _ => Ok(result),
        }
    }).unwrap_or_else(|early_return| early_return)
}

fn eval_if(
    condition: Option<Box<Node>>,
    consequence: Option<Box<Node>>,
    alternative: Option<Box<Node>>,
    env: Env,
) -> Object {
    let cond_val = condition.map(|c| eval(*c, Rc::clone(&env))).unwrap_or(NULL);
    
    if cond_val.is_error() {
        return cond_val;
    }
    
    if is_truthy(&cond_val) {
        consequence.map(|c| eval(*c, env)).unwrap_or(NULL)
    } else {
        alternative.map(|a| eval(*a, env)).unwrap_or(NULL)
    }
}

fn eval_return(return_value: Option<Box<Node>>, env: Env) -> Object {
    return_value
        .map(|val| {
            let result = eval(*val, env);
            if result.is_error() {
                result
            } else {
                Object::ReturnValue(Box::new(result))
            }
        })
        .unwrap_or(NULL)
}

fn eval_let(name: Option<Box<Node>>, value: Option<Box<Node>>, env: Env) -> Object {
    let name_str = match name.map(|n| *n) {
        Some(Node::Identifier { name }) => name,
        _ => return Object::Error("let statement name must be an identifier".to_string()),
    };
    
    let value_obj = value
        .map(|v| eval(*v, Rc::clone(&env)))
        .unwrap_or(Object::Null);
    
    if value_obj.is_error() {
        return value_obj;
    }
    
    env.borrow_mut().set(&name_str, value_obj.clone());
    value_obj
}

fn eval_identifier(name: String, env: Env) -> Object {
    debug!("Evaluating identifier: {}", name);
    env.borrow()
        .get(&name)
        .unwrap_or_else(|| Object::Error(format!("identifier not found: {}", name)))
}

fn eval_call(function: Option<Box<Node>>, arguments: Vec<Node>, env: Env) -> Object {
    let func = function
        .map(|f| eval(*f, Rc::clone(&env)))
        .unwrap_or(NULL);
    
    if func.is_error() {
        return func;
    }
    
    // Evaluate arguments, short-circuit on error
    let args: Result<Vec<Object>, Object> = arguments
        .into_iter()
        .map(|arg| {
            let result = eval(arg, Rc::clone(&env));
            if result.is_error() {
                Err(result)
            } else {
                Ok(result)
            }
        })
        .collect();
    
    match args {
        Err(err) => err,
        Ok(args) => apply_function(func, args),
    }
}

fn apply_function(function: Object, args: Vec<Object>) -> Object {
    match function {
        Object::Function { parameters, body, env: func_env } => {
            let extended_env = extend_function_env(&parameters, args, func_env);
            
            let result = body
                .map(|b| eval(*b, extended_env))
                .unwrap_or(NULL);
            
            unwrap_return_value(result)
        }
        _ => Object::Error(format!("not a function: {}", function.type_name())),
    }
}

fn extend_function_env(parameters: &[Node], args: Vec<Object>, outer: Env) -> Env {
    let extended_env = Environment::new_enclosed(Rc::clone(&outer));
    
    for (param, arg) in parameters.iter().zip(args.into_iter()) {
        if let Node::Identifier { name } = param {
            extended_env.borrow_mut().set(name, arg);
        }
    }
    
    extended_env
}

fn unwrap_return_value(obj: Object) -> Object {
    match obj {
        Object::ReturnValue(value) => *value,
        _ => obj,
    }
}

fn is_truthy(obj: &Object) -> bool {
    !matches!(obj, Object::Boolean(false) | Object::Null)
}

fn native_bool_to_boolean_object(input: bool) -> Object {
    if input { TRUE } else { FALSE }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
use crate::{lexer::Lexer, parser::Parser};

#[cfg(test)]
use tracing_test::traced_test;

/// Helper to run evaluation tests from source code
#[cfg(test)]
fn test_eval(input: &str) -> Object {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    let env = Environment::new();
    eval(program, env)
}

#[rstest]
#[case(Node::IntegerLiteral { value: 5 }, Object::Integer(5))]
#[case(Node::IntegerLiteral { value: 10 }, Object::Integer(10))]
#[case(Node::BooleanLiteral { value: true }, TRUE)]
#[case(Node::BooleanLiteral { value: false }, FALSE)]
fn test_eval_literals(#[case] input: Node, #[case] expected: Object) {
    let env = Environment::new();
    let evaluated = eval(input, env);
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
    assert_eq!(test_eval(input), expected);
}

#[rstest]
#[case("5", Object::Integer(5))]
#[case("10", Object::Integer(10))]
#[case("-5", Object::Integer(-5))]
#[case("-10", Object::Integer(-10))]
fn test_minus_prefix_operator(#[case] input: &str, #[case] expected: Object) {
    assert_eq!(test_eval(input), expected);
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
    match test_eval(input) {
        Object::Integer(value) => assert_eq!(value, expected),
        other => panic!("object is not Integer. got={}", other),
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
    assert_eq!(test_eval(input), expected);
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
    assert_eq!(test_eval(input), expected);
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
    assert_eq!(test_eval(input), expected);
}

#[rstest]
#[case("return 10;", Object::Integer(10))]
#[case("return 10; 9;", Object::Integer(10))]
#[case("return 2 * 5; 9;", Object::Integer(10))]
#[case("9; return 2 * 5; 9;", Object::Integer(10))]
#[case("if (10 > 1) { if (10 > 1) { return 10; } return 1; }", Object::Integer(10))]
fn test_return_statements(#[case] input: &str, #[case] expected: Object) {
    assert_eq!(test_eval(input), expected);
}

#[rstest]
#[case("5 + true;", "type mismatch: INTEGER + BOOLEAN")]
#[case("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN")]
#[case("-true", "unknown operator: -BOOLEAN")]
#[case("true + false;", "unknown operator: BOOLEAN + BOOLEAN")]
#[case("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN")]
#[case("if (10 > 1) { true + false; }", "unknown operator: BOOLEAN + BOOLEAN")]
#[case("if (10 > 1) { if (10 > 1) { return true + false; } return 1; }", "unknown operator: BOOLEAN + BOOLEAN")]
#[case("foobar", "identifier not found: foobar")]
fn test_error_handling(#[case] input: &str, #[case] expected_message: &str) {
    match test_eval(input) {
        Object::Error(message) => assert_eq!(message, expected_message),
        other => panic!("no error object returned. got={}", other),
    }
}

#[rstest]
#[traced_test]
#[case("let a = 5; a;", Object::Integer(5))]
#[case("let a = 5 * 5; a;", Object::Integer(25))]
#[case("let a = 5; let b = a; b;", Object::Integer(5))]
#[case("let a = 5; let b = a; let c = a + b + 5; c;", Object::Integer(15))]
fn test_let_statements(#[case] input: &str, #[case] expected: Object) {
    assert_eq!(test_eval(input), expected);
}

#[rstest]
#[case("let identity = fn(x) { x; }; identity(5);", Object::Integer(5))]
#[case("let identity = fn(x) { return x; }; identity(5);", Object::Integer(5))]
#[case("let double = fn(x) { x * 2; }; double(5);", Object::Integer(10))]
#[case("let add = fn(x, y) { x + y; }; add(5, 5);", Object::Integer(10))]
#[case("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", Object::Integer(20))]
#[case("fn(x) { x; }(5)", Object::Integer(5))]
fn test_function_application(#[case] input: &str, #[case] expected: Object) {
    assert_eq!(test_eval(input), expected);
}

#[rstest]
#[traced_test]
#[case("let newAdder = fn(x) { fn(y) { x + y }; }; let addTwo = newAdder(2); addTwo(2);", Object::Integer(4))]
#[case("let counter = fn(x) { if (x > 10) { return true; } else { let foobar = 9999; counter(x + 1); } }; counter(0);", TRUE)]
fn test_closures(#[case] input: &str, #[case] expected: Object) {
    assert_eq!(test_eval(input), expected);
}

#[rstest]
#[case("let countdown = fn(n) { if (n == 0) { return 0; } else { countdown(n - 1); } }; countdown(50);", Object::Integer(0))]
#[case("let countdown = fn(n) { if (n == 0) { return 0; } else { countdown(n - 1); } }; countdown(100);", Object::Integer(0))]
fn test_deep_recursion(#[case] input: &str, #[case] expected: Object) {
    assert_eq!(test_eval(input), expected);
}
