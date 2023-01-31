use crate::token::Token;

/// An identifier token.
///
/// Grammar: `/[_a-zA-Z][_a-zA-Z0-9]*/`
#[derive(Debug, Clone)]
pub struct Identifier<'s> {
    pub value: &'s str,
    pub token: &'s Token<'s>,
}

/// A method identifier.
///
/// This allows the `constructor` keyword to be used in certain identifier positions.
///
/// Grammar: [Identifier] | `constructor`
#[derive(Debug, Clone)]
pub enum MethodIdentifier<'s> {
    Identifier(Identifier<'s>),
    Constructor(&'s Token<'s>),
}
