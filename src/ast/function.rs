use crate::ast::{
    Expression, Identifier, SeparatedList1, SeparatedListTrailing0, Statement, Type, VarInitializer,
};
use crate::token::Token;

// FunctionEnvironment? `(` FunctionArgs `)` FunctionCaptures? Statement
#[derive(Debug, Clone)]
pub struct FunctionDeclaration<'s> {
    pub environment: Option<FunctionEnvironment<'s>>,
    pub open: &'s Token<'s>,
    pub args: FunctionArgs<'s>,
    pub close: &'s Token<'s>,
    pub captures: Option<FunctionCaptures<'s>>,
    pub body: Box<Statement<'s>>,
}

// `[` Expression `]`
#[derive(Debug, Clone)]
pub struct FunctionEnvironment<'s> {
    pub open: &'s Token<'s>,
    pub value: Box<Expression<'s>>,
    pub close: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub enum FunctionArgs<'s> {
    // SeparatedListTrailing0<FunctionArg `,`>
    NonVariable {
        args: SeparatedListTrailing0<'s, FunctionArg<'s>>,
    },

    // `...`
    EmptyVariable {
        vararg: &'s Token<'s>,
    },

    // SeparatedList1<FunctionArg `,`> `,` `...`
    NonEmptyVariable {
        args: SeparatedList1<'s, FunctionArg<'s>>,
        comma: &'s Token<'s>,
        vararg: &'s Token<'s>,
    },
}

// Type? Identifier VarInitializer?
#[derive(Debug, Clone)]
pub struct FunctionArg<'s> {
    pub ty: Option<Type<'s>>,
    pub name: Identifier<'s>,
    pub initializer: Option<VarInitializer<'s>>,
}

// `:` `(` SeparatedListTrailing0<Identifier `,`> `)`
#[derive(Debug, Clone)]
pub struct FunctionCaptures<'s> {
    pub colon: &'s Token<'s>,
    pub open: &'s Token<'s>,
    pub names: SeparatedListTrailing0<'s, Identifier<'s>>,
    pub close: &'s Token<'s>,
}

// Type Identifier? VarInitializer?
#[derive(Debug, Clone)]
pub struct FunctionRefArg<'s> {
    pub ty: Type<'s>,
    pub name: Option<Identifier<'s>>,
    pub initializer: Option<VarInitializer<'s>>,
}
