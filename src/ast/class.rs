use crate::ast::{
    Expression, FunctionDeclaration, Identifier, MethodIdentifier, TableExpression, Type,
    VarInitializer,
};
use crate::token::Token;

// ClassExtends? `{` ClassMember+ `}`
#[derive(Debug, Clone)]
pub struct ClassDeclaration<'s> {
    pub extends: Option<ClassExtends<'s>>,
    pub open: &'s Token<'s>,
    pub members: Vec<ClassMember<'s>>,
    pub close: &'s Token<'s>,
}

// `extends` Expression
#[derive(Debug, Clone)]
pub struct ClassExtends<'s> {
    pub extends: &'s Token<'s>,
    pub name: Box<Expression<'s>>,
}

// ClassMemberAttributes? ClassMemberType
// ClassMemberAttributes = `</` TableSlot+ `/>`
#[derive(Debug, Clone)]
pub struct ClassMember<'s> {
    pub attributes: Option<TableExpression<'s>>,
    pub ty: ClassMemberType<'s>,
}

#[derive(Debug, Clone)]
pub enum ClassMemberType<'s> {
    // `static`? Identifier `=` Expression `;`?
    Property {
        static_: Option<&'s Token<'s>>,
        name: Identifier<'s>,
        initializer: VarInitializer<'s>,
        semicolon: Option<&'s Token<'s>>,
    },

    // `static`? `[` Expression `]` `=` Expression `;`?
    ComputedProperty {
        static_: Option<&'s Token<'s>>,
        open: &'s Token<'s>,
        name: Box<Expression<'s>>,
        close: &'s Token<'s>,
        initializer: VarInitializer<'s>,
        semicolon: Option<&'s Token<'s>>,
    },

    // `constructor` FunctionDeclaration
    Constructor {
        constructor: &'s Token<'s>,
        declaration: Box<FunctionDeclaration<'s>>,
    },

    // Type? `function` IdentifierOrConstructor FunctionDeclaration
    Function {
        return_type: Option<Type<'s>>,
        function: &'s Token<'s>,
        name: MethodIdentifier<'s>,
        declaration: Box<FunctionDeclaration<'s>>,
    },
}
