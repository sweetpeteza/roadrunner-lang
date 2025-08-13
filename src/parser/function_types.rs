use crate::{ast::traits::Expression, parser::parser::Parser};

pub type PrefixParseFn = fn(&Parser) -> Option<Box<dyn Expression>>;
pub type InfixParseFn = fn(&Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;
