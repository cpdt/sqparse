use crate::ast::{Identifier, VarInitializer};
use crate::token::Token;

/// Entry in an [`EnumStatement`].
///
/// Grammar: [Identifier] [VarInitializer]? `,`?
///
/// [`EnumStatement`]: crate::ast::EnumDefinitionStatement
#[derive(Debug, Clone)]
pub struct EnumEntry<'s> {
    pub name: Identifier<'s>,
    pub initializer: Option<VarInitializer<'s>>,
    pub comma: Option<&'s Token<'s>>,
}
