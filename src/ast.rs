#[derive(Debug, PartialEq)]
pub enum Node {
    // Program variants
    Program {
        statements: Vec<Node>,
    },

    // Expression variants with struct-like syntax
    IntegerLiteral {
        value: i64,
    },
    Identifier {
        name: String,
    },
    Prefix {
        operator: String,
        right: Option<Box<Node>>,
    },
    Infix {
        left: Option<Box<Node>>,
        operator: String,
        right: Option<Box<Node>>,
    },
    BooleanLiteral {
        value: bool,
    },
    If {
        condition: Option<Box<Node>>,
        consequence: Option<Box<Node>>,
        alternative: Option<Box<Node>>,
    },
    Function {
        parameters: Vec<Node>,
        body: Option<Box<Node>>,
    },
    Call {
        function: Option<Box<Node>>,
        arguments: Vec<Node>,
    },

    // Statement variants
    Let {
        name: Option<Box<Node>>,
        value: Option<Box<Node>>,
    },
    Return {
        return_value: Option<Box<Node>>,
    },
    ExprStmt {
        expression: Option<Box<Node>>,
    },
    Block {
        statements: Vec<Node>,
    },
}

impl Node {
    pub fn string(&self) -> String {
        match self {
            Node::Program { statements } => statements
                .iter()
                .map(|s| s.string())
                .collect::<Vec<String>>()
                .join(""),
            Node::IntegerLiteral { value } => value.to_string(),
            Node::Identifier { name } => name.clone(),
            Node::Prefix { operator, right } => {
                format!(
                    "({}{})",
                    operator,
                    right.as_ref().map_or("".to_string(), |node| node.string())
                )
            }
            Node::Infix {
                left,
                operator,
                right,
            } => {
                format!(
                    "({} {} {})",
                    left.as_ref().map_or("".to_string(), |node| node.string()),
                    operator,
                    right.as_ref().map_or("".to_string(), |node| node.string())
                )
            }
            Node::BooleanLiteral { value } => value.to_string(),
            Node::If {
                condition,
                consequence,
                alternative,
            } => {
                format!(
                    "if {} {} else {}",
                    condition.as_ref().map_or("".to_string(), |c| c.string()),
                    consequence.as_ref().map_or("".to_string(), |c| c.string()),
                    alternative.as_ref().map_or("".to_string(), |a| a.string())
                )
            }
            Node::Function { parameters, body } => {
                format!(
                    "fn({}) {{{}}}",
                    parameters
                        .iter()
                        .map(|p| p.string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    body.as_ref().map_or("".to_string(), |b| b.string())
                )
            }
            Node::Call {
                function,
                arguments,
            } => {
                format!(
                    "{}({})",
                    function.as_ref().map_or("".to_string(), |f| f.string()),
                    arguments
                        .iter()
                        .map(|a| a.string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Node::Let { name, value } => {
                format!(
                    "let {} = {}",
                    name.as_ref().map_or("".to_string(), |n| n.string()),
                    value.as_ref().map_or("".to_string(), |v| v.string())
                )
            }
            Node::Return { return_value } => {
                format!(
                    "return {}",
                    return_value.as_ref().map_or("".to_string(), |v| v.string())
                )
            }
            Node::ExprStmt { expression } => {
                expression.as_ref().map_or("".to_string(), |e| e.string())
            }
            Node::Block { statements } => statements
                .iter()
                .map(|s| s.string())
                .collect::<Vec<String>>()
                .join("\n"),
        }
    }

    pub fn token_literal(&self) -> String {
        match self {
            Node::IntegerLiteral { value } => value.to_string(),
            Node::Identifier { name } => name.clone(),
            Node::BooleanLiteral { value } => value.to_string(),
            _ => "".to_string(),
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Precedence {
    Lowest = 0,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}
