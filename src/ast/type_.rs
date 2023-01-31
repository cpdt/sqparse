use crate::ast::{
    Expression, FunctionRefArg, Identifier, SeparatedListTrailing0, SeparatedListTrailing1,
    StructDeclaration,
};
use crate::token::Token;

/// A type.
///
/// Many types are recursive, containing other types. Unlike [expressions] types do not have
/// precedence, and are all left-associative.
///
/// This means you can always decompose a type like this:
/// ```text
/// table<int>& ornull
/// ^    ^^^^^^^^^^^^^ modifier types
/// | base type
/// ```
///
/// [expressions]: Expression
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

/// A `local` type.
///
/// Grammar: `local`
#[derive(Debug, Clone)]
pub struct LocalType<'s> {
    pub local: &'s Token<'s>,
}

/// A `var` type.
///
/// Grammar: `var`
#[derive(Debug, Clone)]
pub struct VarType<'s> {
    pub var: &'s Token<'s>,
}

/// A named type.
///
/// Grammar: [Identifier]
#[derive(Debug, Clone)]
pub struct PlainType<'s> {
    pub name: Identifier<'s>,
}

/// An anonymous struct type.
///
/// Grammar: `struct` [StructDeclaration]
#[derive(Debug, Clone)]
pub struct StructType<'s> {
    pub struct_: &'s Token<'s>,
    pub declaration: StructDeclaration<'s>,
}

/// An array type.
///
/// Grammar: [Type] `[` [Expression] `]`
#[derive(Debug, Clone)]
pub struct ArrayType<'s> {
    pub base: Box<Type<'s>>,
    pub open: &'s Token<'s>,
    pub len: Box<Expression<'s>>,
    pub close: &'s Token<'s>,
}

/// A generic type.
///
/// Grammar: [Type] `<` [SeparatedListTrailing1]<[Type] `,`> `>`
#[derive(Debug, Clone)]
pub struct GenericType<'s> {
    pub base: Box<Type<'s>>,
    pub open: &'s Token<'s>,
    pub params: SeparatedListTrailing1<'s, Type<'s>>,
    pub close: &'s Token<'s>,
}

/// A function reference type.
///
/// Grammar: [Type]? `functionref` `(` [SeparatedListTrailing0]<[FunctionRefArg] `,`> `)`
#[derive(Debug, Clone)]
pub struct FunctionRefType<'s> {
    pub return_type: Option<Box<Type<'s>>>,
    pub functionref: &'s Token<'s>,
    pub open: &'s Token<'s>,
    pub args: SeparatedListTrailing0<'s, FunctionRefArg<'s>>,
    pub close: &'s Token<'s>,
}

/// A reference type.
///
/// Grammar: [Type] `&`
#[derive(Debug, Clone)]
pub struct ReferenceType<'s> {
    pub base: Box<Type<'s>>,
    pub reference: &'s Token<'s>,
}

/// A nullable type.
///
/// Grammar: [Type] `ornull`
#[derive(Debug, Clone)]
pub struct NullableType<'s> {
    pub base: Box<Type<'s>>,
    pub ornull: &'s Token<'s>,
}
