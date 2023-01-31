use crate::ast::{Expression, Identifier};
use crate::token::Token;

/// Variable declaration with an optional initializer.
///
/// Grammar: [Identifier] [VarInitializer]?
#[derive(Debug, Clone)]
pub struct VarDeclaration<'s> {
    pub name: Identifier<'s>,
    pub initializer: Option<VarInitializer<'s>>,
}

/// Initializer for some kind of variable or constant.
///
/// Grammar: `=` [Expression]
#[derive(Debug, Clone)]
pub struct VarInitializer<'s> {
    pub assign: &'s Token<'s>,
    pub value: Box<Expression<'s>>,
}
