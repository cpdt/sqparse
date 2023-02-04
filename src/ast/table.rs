use crate::ast::slot::Slot;
use crate::ast::Expression;
use crate::token::Token;

/// Slot in a [`TableExpression`] with an optional separator.
///
/// Grammar: [TableSlotType] `,`?
///
/// [`TableExpression`]: crate::ast::TableExpression
#[derive(Debug, Clone)]
pub struct TableSlot<'s> {
    pub ty: TableSlotType<'s>,
    pub comma: Option<&'s Token<'s>>,
}

/// Slot in a [`TableExpression`].
///
/// [`TableExpression`]: crate::ast::TableExpression
#[derive(Debug, Clone)]
pub enum TableSlotType<'s> {
    /// Grammar: [Slot]
    Slot(Slot<'s>),

    /// JSON property slot.
    ///
    /// Grammar: "literal string" `:` [Expression]
    JsonProperty {
        name: &'s str,
        name_token: &'s Token<'s>,
        colon: &'s Token<'s>,
        value: Box<Expression<'s>>,
    },
}
