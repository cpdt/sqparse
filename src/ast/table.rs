use crate::ast::{
    Expression, FunctionDeclaration, Identifier, LiteralExpression, Type, VarInitializer,
};
use crate::token::Token;

/// Slot in a [`TableExpression`] with an optional separator.
///
/// Grammar: [TableSlotType] `,`?
///
/// [`TableExpression`]: crate::ast::TableExpression
#[derive(Debug, Clone)]
pub struct TableSlot<'s> {
    pub ty: TableSlotType<'s>,
    pub separator: Option<&'s Token<'s>>,
}

/// Slot in a [`TableExpression`].
///
/// [`TableExpression`]: crate::ast::TableExpression
#[derive(Debug, Clone)]
pub enum TableSlotType<'s> {
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

    /// JSON property slot.
    ///
    /// Grammar: [LiteralExpression] `:` [Expression]
    JsonProperty {
        name: LiteralExpression<'s>,
        colon: &'s Token<'s>,
        value: Box<Expression<'s>>,
    },

    /// Function slot.
    ///
    /// Grammar: [Type]? `function` [Identifier] [FunctionDeclaration]
    Function {
        return_type: Option<Type<'s>>,
        function: &'s Token<'s>,
        name: Identifier<'s>,
        declaration: Box<FunctionDeclaration<'s>>,
    },
}
