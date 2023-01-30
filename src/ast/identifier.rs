use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Identifier<'s> {
    pub value: &'s str,
    pub token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub enum MethodIdentifier<'s> {
    Identifier(Identifier<'s>),
    Constructor(&'s Token<'s>),
}
