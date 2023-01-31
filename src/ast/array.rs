use crate::ast::Expression;
use crate::token::Token;

/// Value in an array.
///
/// Grammar: [Expression] `,`?
#[derive(Debug, Clone)]
pub struct ArrayValue<'s> {
    pub value: Box<Expression<'s>>,
    pub separator: Option<&'s Token<'s>>,
}
