use crate::ast::{Identifier, Type, VarInitializer};
use crate::token::Token;

/// Anonymous definition of a struct.
///
/// Grammar: `{` [StructProperty]* `}`
#[derive(Debug, Clone)]
pub struct StructDefinition<'s> {
    pub open: &'s Token<'s>,
    pub properties: Vec<StructProperty<'s>>,
    pub close: &'s Token<'s>,
}

/// Property of a struct in a [`StructDefinition`].
///
/// Grammar: [Type] [Identifier] [VarInitializer]? `,`?
#[derive(Debug, Clone)]
pub struct StructProperty<'s> {
    pub type_: Type<'s>,
    pub name: Identifier<'s>,
    pub initializer: Option<VarInitializer<'s>>,
    pub comma: Option<&'s Token<'s>>,
}
