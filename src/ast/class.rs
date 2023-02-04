use crate::ast::slot::Slot;
use crate::ast::{Expression, TableExpression};
use crate::token::Token;

/// Anonymous definition of a class.
///
/// Grammar: [ClassExtends]? `{` [ClassMember]* `}`
#[derive(Debug, Clone)]
pub struct ClassDefinition<'s> {
    pub extends: Option<ClassExtends<'s>>,
    pub open: &'s Token<'s>,
    pub members: Vec<ClassMember<'s>>,
    pub close: &'s Token<'s>,
}

/// Optional extends part of [`ClassDefinition`].
///
/// Grammar: `extends` [Expression]
#[derive(Debug, Clone)]
pub struct ClassExtends<'s> {
    pub extends: &'s Token<'s>,
    pub name: Box<Expression<'s>>,
}

/// Member of a [`ClassDefinition`] with an optional attribute table.
///
/// Grammar: (`</` [TableSlot]* `/>`)? `static`? [Slot] `;`?
///
/// [TableSlot]: crate::ast::TableSlot
#[derive(Debug, Clone)]
pub struct ClassMember<'s> {
    pub attributes: Option<TableExpression<'s>>,
    pub static_: Option<&'s Token<'s>>,
    pub slot: Slot<'s>,
    pub semicolon: Option<&'s Token<'s>>,
}
