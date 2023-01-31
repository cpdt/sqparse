use crate::ast::{
    Expression, FunctionDeclaration, Identifier, MethodIdentifier, TableExpression, Type,
    VarInitializer,
};
use crate::token::Token;

/// Anonymous declaration of a class.
///
/// Grammar: [ClassExtends]? `{` [ClassMember]* `}`
#[derive(Debug, Clone)]
pub struct ClassDeclaration<'s> {
    pub extends: Option<ClassExtends<'s>>,
    pub open: &'s Token<'s>,
    pub members: Vec<ClassMember<'s>>,
    pub close: &'s Token<'s>,
}

/// Optional extends part of [`ClassDeclaration`].
///
/// Grammar: `extends` [Expression]
#[derive(Debug, Clone)]
pub struct ClassExtends<'s> {
    pub extends: &'s Token<'s>,
    pub name: Box<Expression<'s>>,
}

/// Member of a [`ClassDeclaration`] with an optional attribute table.
///
/// Grammar: (`</` [TableSlot]+ `/>`)? [ClassMemberType]
///
/// [TableSlot]: crate::ast::TableSlot
#[derive(Debug, Clone)]
pub struct ClassMember<'s> {
    pub attributes: Option<TableExpression<'s>>,
    pub ty: ClassMemberType<'s>,
}

/// Member of a [`ClassDeclaration`].
#[derive(Debug, Clone)]
pub enum ClassMemberType<'s> {
    /// Class property.
    ///
    /// Grammar: `static`? [Identifier] `=` [Expression] `;`?
    Property {
        static_: Option<&'s Token<'s>>,
        name: Identifier<'s>,
        initializer: VarInitializer<'s>,
        semicolon: Option<&'s Token<'s>>,
    },

    /// Computed class property.
    ///
    /// Grammar: `static`? `[` [Expression] `]` `=` [Expression] `;`?
    ComputedProperty {
        static_: Option<&'s Token<'s>>,
        open: &'s Token<'s>,
        name: Box<Expression<'s>>,
        close: &'s Token<'s>,
        initializer: VarInitializer<'s>,
        semicolon: Option<&'s Token<'s>>,
    },

    /// Class constructor.
    ///
    /// Grammar: `constructor` [FunctionDeclaration]
    Constructor {
        constructor: &'s Token<'s>,
        declaration: Box<FunctionDeclaration<'s>>,
    },

    /// Class method.
    ///
    /// Grammar: [Type]? `function` [MethodIdentifier] [FunctionDeclaration]
    Function {
        return_type: Option<Type<'s>>,
        function: &'s Token<'s>,
        name: MethodIdentifier<'s>,
        declaration: Box<FunctionDeclaration<'s>>,
    },
}
