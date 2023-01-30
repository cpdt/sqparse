use crate::ast::{
    Expression, Identifier, SeparatedList0, SeparatedList1, StructDeclaration, VarInitializer,
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

//

#[derive(Debug, Clone)]
pub struct LocalType<'s> {
    pub token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct VarType<'s> {
    pub token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct PlainType<'s> {
    pub identifier: Identifier<'s>,
}

#[derive(Debug, Clone)]
pub struct ArrayType<'s> {
    pub base: Box<Type<'s>>,
    pub len: Box<Expression<'s>>,

    pub open_len_token: &'s Token<'s>,
    pub close_len_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct GenericType<'s> {
    pub base: Box<Type<'s>>,
    pub params: SeparatedList1<'s, Type<'s>>,

    pub open_params_token: &'s Token<'s>,
    pub trailing_separator_token: Option<&'s Token<'s>>,
    pub close_params_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct FunctionRefType<'s> {
    pub ret_type: Option<Box<Type<'s>>>,
    pub params: SeparatedList0<'s, FunctionParam<'s>>,

    pub functionref_token: &'s Token<'s>,
    pub open_params_token: &'s Token<'s>,
    pub trailing_separator_token: Option<&'s Token<'s>>,
    pub close_params_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct StructType<'s> {
    pub declaration: StructDeclaration<'s>,

    pub struct_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct ReferenceType<'s> {
    pub base: Box<Type<'s>>,

    pub ref_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct NullableType<'s> {
    pub base: Box<Type<'s>>,

    pub nullable_token: &'s Token<'s>,
}

//

#[derive(Debug, Clone)]
pub struct FunctionParam<'s> {
    pub param_type: Type<'s>,
    pub param_name: Option<Identifier<'s>>,
    pub initializer: Option<VarInitializer<'s>>,
}
