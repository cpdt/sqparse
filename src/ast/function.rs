use crate::ast::{
    Expression, Identifier, SeparatedList1, SeparatedListTrailing0, StatementType, Type,
    VarInitializer,
};
use crate::token::Token;

/// Anonymous function definition in a function literal, function definition, class method, or table function.
///
/// Grammar: [FunctionEnvironment]? `(` [FunctionParams] `)` [FunctionCaptures]? [StatementType]
#[derive(Debug, Clone)]
pub struct FunctionDefinition<'s> {
    pub environment: Option<FunctionEnvironment<'s>>,
    pub open: &'s Token<'s>,
    pub params: FunctionParams<'s>,
    pub close: &'s Token<'s>,
    pub captures: Option<FunctionCaptures<'s>>,
    pub body: Box<StatementType<'s>>,
}

/// Environment that is bound to a [`FunctionDefinition`].
///
/// Grammar: `[` [Expression] `]`
#[derive(Debug, Clone)]
pub struct FunctionEnvironment<'s> {
    pub open: &'s Token<'s>,
    pub value: Box<Expression<'s>>,
    pub close: &'s Token<'s>,
}

/// Parameter definition in a [`FunctionDefinition`].
#[derive(Debug, Clone)]
pub enum FunctionParams<'s> {
    /// Non-variable argument list.
    ///
    /// Grammar: [SeparatedListTrailing0]<[FunctionParam] `,`>
    NonVariable {
        params: SeparatedListTrailing0<'s, FunctionParam<'s>>,
    },

    /// Variable-length argument list with no named arguments.
    ///
    /// Grammar: `...`
    EmptyVariable { vararg: &'s Token<'s> },

    /// Variable-length argument list with some named arguments.
    ///
    /// Grammar: [SeparatedList1]<[FunctionParam] `,`> `,` `...`
    NonEmptyVariable {
        params: SeparatedList1<'s, FunctionParam<'s>>,
        comma: &'s Token<'s>,
        vararg: &'s Token<'s>,
    },
}

/// Parameter definition in a [`FunctionParams`] list.
///
/// Grammar: [Type]? [Identifier] [VarInitializer]?
#[derive(Debug, Clone)]
pub struct FunctionParam<'s> {
    pub type_: Option<Type<'s>>,
    pub name: Identifier<'s>,
    pub initializer: Option<VarInitializer<'s>>,
}

/// List of captured variables (aka free variables) in a [`FunctionDefinition`].
///
/// Grammar: `:` `(` [SeparatedListTrailing0]<[Identifier] `,`> `)`
#[derive(Debug, Clone)]
pub struct FunctionCaptures<'s> {
    pub colon: &'s Token<'s>,
    pub open: &'s Token<'s>,
    pub names: SeparatedListTrailing0<'s, Identifier<'s>>,
    pub close: &'s Token<'s>,
}

/// Parameter definition in a [`FunctionRefType`].
///
/// Grammar: [Type] [Identifier]? [VarInitializer]?
///
/// [`FunctionRefType`]: crate::ast::FunctionRefType
#[derive(Debug, Clone)]
pub struct FunctionRefParam<'s> {
    pub type_: Type<'s>,
    pub name: Option<Identifier<'s>>,
    pub initializer: Option<VarInitializer<'s>>,
}

/// Argument in a [`CallExpression`].
///
/// Grammar: [Expression] `,`?
///
/// [`CallExpression`]: crate::ast::CallExpression
#[derive(Debug, Clone)]
pub struct CallArgument<'s> {
    pub value: Box<Expression<'s>>,
    pub comma: Option<&'s Token<'s>>,
}
