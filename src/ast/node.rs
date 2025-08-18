use std::marker::PhantomData;

// Marker types
#[derive(Debug, PartialEq)]
pub struct ExprMarker;
#[derive(Debug, PartialEq)]
pub struct StmtMarker;
#[derive(Debug, PartialEq)]
pub struct ProgramMarker;

pub struct NodeMarker;

// Type aliases
pub type Expression = Node<ExprMarker>;
pub type Statement = Node<StmtMarker>;
pub type Program = Node<ProgramMarker>;
pub type AnyNode = Node<NodeMarker>;

#[derive(Debug, PartialEq)]
pub enum Node<T = NodeMarker> {
    // Program variants
    Program {
        statements: Vec<Statement>,
        _marker: PhantomData<T>,
    },

    // Expression variants with struct-like syntax
    IntegerLiteral {
        value: i64,
        _marker: PhantomData<T>,
    },
    Identifier {
        name: String,
        _marker: PhantomData<T>,
    },
    Prefix {
        operator: String,
        right: Option<Box<Expression>>,
        _marker: PhantomData<T>,
    },
    Infix {
        left: Option<Box<Expression>>,
        operator: String,
        right: Option<Box<Expression>>,
        _marker: PhantomData<T>,
    },
    BooleanLiteral {
        value: bool,
        _marker: PhantomData<T>,
    },
    If {
        condition: Option<Box<Expression>>,
        consequence: Option<Box<Statement>>,
        alternative: Option<Box<Statement>>,
        _marker: PhantomData<T>,
    },
    Function {
        parameters: Vec<Expression>,
        body: Option<Box<Statement>>,
        _marker: PhantomData<T>,
    },
    Call {
        function: Option<Box<Expression>>,
        arguments: Vec<Expression>,
        _marker: PhantomData<T>,
    },

    // Statement variants
    Let {
        name: String,
        value: Option<Box<Expression>>,
        _marker: PhantomData<T>,
    },
    Return {
        return_value: Option<Box<Expression>>,
        _marker: PhantomData<T>,
    },
    ExprStmt {
        expression: Option<Box<Expression>>,
        _marker: PhantomData<T>,
    },
    Block {
        statements: Vec<Statement>,
        _marker: PhantomData<T>,
    },
}

impl Expression {
    pub fn integer_literal(value: i64) -> Self {
        Node::IntegerLiteral {
            value,
            _marker: PhantomData,
        }
    }

    pub fn identifier(name: String) -> Self {
        Node::Identifier {
            name,
            _marker: PhantomData,
        }
    }

    pub fn prefix(operator: String, right: Option<Box<Expression>>) -> Self {
        Node::Prefix {
            operator,
            right,
            _marker: PhantomData,
        }
    }

    pub fn infix(
        left: Option<Box<Expression>>,
        operator: String,
        right: Option<Box<Expression>>,
    ) -> Self {
        Node::Infix {
            left,
            operator,
            right,
            _marker: PhantomData,
        }
    }

    pub fn boolean_literal(value: bool) -> Self {
        Node::BooleanLiteral {
            value,
            _marker: PhantomData,
        }
    }

    pub fn if_expr(
        condition: Option<Box<Expression>>,
        consequence: Option<Box<Statement>>,
        alternative: Option<Box<Statement>>,
    ) -> Self {
        Node::If {
            condition,
            consequence,
            alternative,
            _marker: PhantomData,
        }
    }

    pub fn function(parameters: Vec<Expression>, body: Option<Box<Statement>>) -> Self {
        Node::Function {
            parameters,
            body,
            _marker: PhantomData,
        }
    }

    pub fn call(function: Option<Box<Expression>>, arguments: Vec<Expression>) -> Self {
        Node::Call {
            function,
            arguments,
            _marker: PhantomData,
        }
    }
}

impl Statement {
    pub fn let_stmt(name: String, value: Option<Box<Expression>>) -> Self {
        Node::Let {
            name,
            value,
            _marker: PhantomData,
        }
    }

    pub fn return_stmt(return_value: Option<Box<Expression>>) -> Self {
        Node::Return {
            return_value,
            _marker: PhantomData,
        }
    }

    pub fn expr_stmt(expression: Option<Box<Expression>>) -> Self {
        Node::ExprStmt {
            expression,
            _marker: PhantomData,
        }
    }

    pub fn block(statements: Vec<Statement>) -> Self {
        Node::Block {
            statements,
            _marker: PhantomData,
        }
    }
}

impl Program {
    pub fn program(statements: Vec<Statement>) -> Self {
        Node::Program {
            statements,
            _marker: PhantomData,
        }
    }
}
