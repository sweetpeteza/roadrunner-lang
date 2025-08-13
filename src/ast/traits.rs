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

impl PartialEq for Box<dyn Expression + 'static> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().string() == other.as_ref().string()
    }
}

impl std::fmt::Debug for Box<dyn Expression + 'static> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Expression")
            .field("string", &self.string())
            .finish()
    }
}

pub trait ExprStatement: Statement {
    fn expression(&self) -> Option<&dyn Expression>;
}
