use crate::ast::{
    Expression, FunctionDeclaration, Identifier, LiteralExpression, Type, VarInitializer,
};
use crate::token::Token;

// TableSlotType `,`?
#[derive(Debug, Clone)]
pub struct TableSlot<'s> {
    pub ty: TableSlotType<'s>,
    pub separator: Option<&'s Token<'s>>,
}

#[derive(Debug, Clone)]
pub enum TableSlotType<'s> {
    // Identifier VarInitializer
    Property {
        name: Identifier<'s>,
        initializer: VarInitializer<'s>,
    },

    // `[` Expression `]` VarInitializer
    ComputedProperty {
        open: &'s Token<'s>,
        name: Box<Expression<'s>>,
        close: &'s Token<'s>,
        initializer: VarInitializer<'s>,
    },

    // LiteralExpression `:` Expression
    JsonProperty {
        name: LiteralExpression<'s>,
        colon: &'s Token<'s>,
        value: Box<Expression<'s>>,
    },

    // Type? `function` Identifier FunctionDeclaration
    Function {
        return_type: Option<Type<'s>>,
        function: &'s Token<'s>,
        name: Identifier<'s>,
        declaration: Box<FunctionDeclaration<'s>>,
    },
}
