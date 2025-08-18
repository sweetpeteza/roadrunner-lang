use std::marker::PhantomData;

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

