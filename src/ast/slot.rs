use crate::ast::{Expression, FunctionDefinition, Identifier, Type, VarInitializer};
use crate::token::Token;

/// Slot in a [`TableExpression`] or [`ClassDefinition`].
///
/// [`TableExpression`]: crate::ast::TableExpression
/// [`ClassDefinition`]: crate::ast::ClassDefinition
#[derive(Debug, Clone)]
pub enum Slot<'s> {
    /// Property slot.
    ///
    /// Grammar: [Identifier] [VarInitializer]
    Property {
        name: Identifier<'s>,
        initializer: VarInitializer<'s>,
    },

    /// Computed property slot.
    ///
    /// Grammar: `[` [Expression] `]` [VarInitializer]
    ComputedProperty {
        open: &'s Token<'s>,
        name: Box<Expression<'s>>,
        close: &'s Token<'s>,
        initializer: VarInitializer<'s>,
    },

    /// Constructor slot.
    ///
    /// While mainly intended for classes, this is also valid in a table due to a quirk in the
    /// vanilla Squirrel parser, but it's just treated as a function named `constructor`.
    ///
    /// Grammar: `function`? `constructor` [FunctionDefinition]
    Constructor {
        function: Option<&'s Token<'s>>,
        constructor: &'s Token<'s>,
        definition: Box<FunctionDefinition<'s>>,
    },

    /// Function slot.
    ///
    /// Grammar: [Type]? `function` [Identifier] [FunctionDefinition]
    Function {
        return_type: Option<Type<'s>>,
        function: &'s Token<'s>,
        name: Identifier<'s>,
        definition: Box<FunctionDefinition<'s>>,
    },
}
