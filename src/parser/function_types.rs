use crate::{ast::expression_types::ExpressionType, parser::parser::Parser};

pub type PrefixParseFn = fn(&Parser) -> Option<ExpressionType>;
pub type InfixParseFn = fn(&Parser, ExpressionType) -> Option<ExpressionType>;
