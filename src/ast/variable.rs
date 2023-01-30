use crate::ast::{Expression, Identifier};
use crate::token::Token;

// Identifier VarInitializer?
#[derive(Debug, Clone)]
pub struct VarDeclaration<'s> {
    pub name: Identifier<'s>,
    pub initializer: Option<VarInitializer<'s>>,
}

// `=` Expression
#[derive(Debug, Clone)]
pub struct VarInitializer<'s> {
    pub assign: &'s Token<'s>,
    pub value: Box<Expression<'s>>,
}
