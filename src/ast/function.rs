use crate::ast::{
    Expression, Identifier, SeparatedList1, SeparatedListTrailing0, Statement, Type, VarInitializer,
};
use crate::token::Token;

/// Anonymous function declaration in a function literal, function definition, class method, or table function.
///
/// Grammar: [FunctionEnvironment]? `(` [FunctionArgs] `)` [FunctionCaptures]? [Statement]
#[derive(Debug, Clone)]
pub struct FunctionDeclaration<'s> {
    pub environment: Option<FunctionEnvironment<'s>>,
    pub open: &'s Token<'s>,
    pub args: FunctionArgs<'s>,
    pub close: &'s Token<'s>,
    pub captures: Option<FunctionCaptures<'s>>,
    pub body: Box<Statement<'s>>,
}

/// Environment that is bound to a [`FunctionDeclaration`].
///
/// Grammar: `[` [Expression] `]`
#[derive(Debug, Clone)]
pub struct FunctionEnvironment<'s> {
    pub open: &'s Token<'s>,
    pub value: Box<Expression<'s>>,
    pub close: &'s Token<'s>,
}

/// Argument declarations in a [`FunctionDeclaration`].
#[derive(Debug, Clone)]
pub enum FunctionArgs<'s> {
    /// Non-variable argument list.
    ///
    /// Grammar: [SeparatedListTrailing0]<[FunctionArg] `,`>
    NonVariable {
        args: SeparatedListTrailing0<'s, FunctionArg<'s>>,
    },

    /// Variable-length argument list with no named arguments.
    ///
    /// Grammar: `...`
    EmptyVariable { vararg: &'s Token<'s> },

    /// Variable-length argument list with some named arguments.
    ///
    /// Grammar: [SeparatedList1]<[FunctionArg] `,`> `,` `...`
    NonEmptyVariable {
        args: SeparatedList1<'s, FunctionArg<'s>>,
        comma: &'s Token<'s>,
        vararg: &'s Token<'s>,
    },
}

/// Argument declaration in a [`FunctionArgs`] list.
///
/// Grammar: [Type]? [Identifier] [VarInitializer]?
#[derive(Debug, Clone)]
pub struct FunctionArg<'s> {
    pub ty: Option<Type<'s>>,
    pub name: Identifier<'s>,
    pub initializer: Option<VarInitializer<'s>>,
}

/// List of captured variables (aka free variables) in a [`FunctionDeclaration`].
///
/// Grammar: `:` `(` [SeparatedListTrailing0]<[Identifier] `,`> `)`
#[derive(Debug, Clone)]
pub struct FunctionCaptures<'s> {
    pub colon: &'s Token<'s>,
    pub open: &'s Token<'s>,
    pub names: SeparatedListTrailing0<'s, Identifier<'s>>,
    pub close: &'s Token<'s>,
}

/// Argument declaration in a [`FunctionRefType`].
///
/// Grammar: [Type] [Identifier]? [VarInitializer]?
///
/// [`FunctionRefType`]: crate::ast::FunctionRefType
#[derive(Debug, Clone)]
pub struct FunctionRefArg<'s> {
    pub ty: Type<'s>,
    pub name: Option<Identifier<'s>>,
    pub initializer: Option<VarInitializer<'s>>,
}
