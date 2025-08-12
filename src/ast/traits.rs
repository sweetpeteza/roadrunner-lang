pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub trait ExpressionStatement: Statement {
    fn expression(&self) -> &dyn Expression;
}
