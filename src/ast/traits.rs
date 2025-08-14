pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}
