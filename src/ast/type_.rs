use crate::ast::{
    Expression, FunctionRefArg, Identifier, SeparatedListTrailing0, SeparatedListTrailing1,
    StructDeclaration,
};
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Type<'s> {
    Local(LocalType<'s>),
    Var(VarType<'s>),
    Plain(PlainType<'s>),
    Array(ArrayType<'s>),
    Generic(GenericType<'s>),
    FunctionRef(FunctionRefType<'s>),
    Struct(StructType<'s>),
    Reference(ReferenceType<'s>),
    Nullable(NullableType<'s>),
}

// `local`
#[derive(Debug, Clone)]
pub struct LocalType<'s> {
    pub local: &'s Token<'s>,
}

// `var`
#[derive(Debug, Clone)]
pub struct VarType<'s> {
    pub var: &'s Token<'s>,
}

// Identifier
#[derive(Debug, Clone)]
pub struct PlainType<'s> {
    pub name: Identifier<'s>,
}

// `struct` StructDeclaration
#[derive(Debug, Clone)]
pub struct StructType<'s> {
    pub struct_: &'s Token<'s>,
    pub declaration: StructDeclaration<'s>,
}

// Type `[` Expression `]`
#[derive(Debug, Clone)]
pub struct ArrayType<'s> {
    pub base: Box<Type<'s>>,
    pub open: &'s Token<'s>,
    pub len: Box<Expression<'s>>,
    pub close: &'s Token<'s>,
}

// Type `<` SeparatedListTrailing1<Type `,`> `>`
#[derive(Debug, Clone)]
pub struct GenericType<'s> {
    pub base: Box<Type<'s>>,
    pub open: &'s Token<'s>,
    pub params: SeparatedListTrailing1<'s, Type<'s>>,
    pub close: &'s Token<'s>,
}

// Type? `functionref` `(` SeparatedListTrailing0<FunctionRefArg `,`> `)`
#[derive(Debug, Clone)]
pub struct FunctionRefType<'s> {
    pub return_type: Option<Box<Type<'s>>>,
    pub functionref: &'s Token<'s>,
    pub open: &'s Token<'s>,
    pub args: SeparatedListTrailing0<'s, FunctionRefArg<'s>>,
    pub close: &'s Token<'s>,
}

// Type `&`
#[derive(Debug, Clone)]
pub struct ReferenceType<'s> {
    pub base: Box<Type<'s>>,
    pub reference: &'s Token<'s>,
}

// Type `ornull`
#[derive(Debug, Clone)]
pub struct NullableType<'s> {
    pub base: Box<Type<'s>>,
    pub ornull: &'s Token<'s>,
}
