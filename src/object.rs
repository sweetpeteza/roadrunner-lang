use crate::{ast::Node, environment::Environment};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
    ReturnValue(Box<Object>),
    Error(String),
    Function {
        parameters: Vec<Node>,
        body: Option<Box<Node>>,
        env: Box<Environment>,
    },
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inspect())
    }
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(value) => value.to_string(),
            Object::Boolean(value) => value.to_string(),
            Object::Null => "null".to_string(),
            Object::ReturnValue(value) => value.as_ref().inspect(),
            Object::Error(error) => error.to_string(),
            Object::Function {
                parameters, body, ..
            } => {
                let mut out = String::new();

                let params: Vec<String> = parameters.iter().map(|p| p.string()).collect();

                let body = body.as_ref().map_or("".to_string(), |b| b.string());

                out.push_str(format!("fn({}) {{\n {} \n}}", params.join(", "), body).as_str());

                out
            }
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Object::Integer(_) => "INTEGER",
            Object::Boolean(_) => "BOOLEAN",
            Object::Null => "NULL",
            Object::ReturnValue(_) => "RETURN_VALUE",
            Object::Error(_) => "ERROR",
            Object::Function { .. } => "FUNCTION_OBJ",
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Object::Error(_))
    }
}
