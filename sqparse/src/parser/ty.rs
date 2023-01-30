use crate::ast::{
    ArrayType, Expression, FunctionParam, FunctionRefType, GenericType, LocalType, NullableType,
    PlainType, ReferenceType, SeparatedList0, SeparatedList1, StructType, Type, VarType,
};
use crate::parser::expression::expression;
use crate::parser::shared::{
    cond_not_line_ending, identifier, perfect, separated_list0, separated_list1,
    struct_declaration, terminal, var_initializer,
};
use crate::parser::TokenList;
use crate::token::TerminalToken;
use crate::{IResult, Token};
use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::multi::fold_many0;
use nom::sequence::{pair, tuple};

pub fn ty<'s>(can_end_line: bool) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, Type<'s>> {
    move |i| {
        let (i, base) = base_type(can_end_line)(i)?;

        fold_many0(
            alt((
                array_type,
                generic_type,
                function_ref_type,
                reference_type,
                nullable_type,
            )),
            move || base.clone(),
            |base, almost_expr| almost_expr.into_type(base),
        )(i)
    }
}

// Base types
fn local_type(i: TokenList) -> IResult<TokenList, LocalType> {
    map(terminal(TerminalToken::Local), |token| LocalType { token })(i)
}

fn var_type(i: TokenList) -> IResult<TokenList, VarType> {
    map(terminal(TerminalToken::Var), |token| VarType { token })(i)
}

fn plain_type(i: TokenList) -> IResult<TokenList, PlainType> {
    map(identifier, |identifier| PlainType { identifier })(i)
}

fn function_param(i: TokenList) -> IResult<TokenList, FunctionParam> {
    map(
        tuple((ty(true), opt(identifier), opt(var_initializer))),
        |(param_type, param_name, initializer)| FunctionParam {
            param_type,
            param_name,
            initializer,
        },
    )(i)
}

fn void_function_ref_type(i: TokenList) -> IResult<TokenList, FunctionRefType> {
    map(
        pair(
            terminal(TerminalToken::FunctionRef),
            perfect(
                "functionref type",
                tuple((
                    terminal(TerminalToken::OpenBracket),
                    separated_list0(TerminalToken::Comma, function_param),
                    opt(terminal(TerminalToken::Comma)),
                    terminal(TerminalToken::CloseBracket),
                )),
            ),
        ),
        |(
            functionref_token,
            (open_params_token, params, trailing_separator_token, close_params_token),
        )| FunctionRefType {
            ret_type: None,
            params,

            functionref_token,
            open_params_token,
            trailing_separator_token,
            close_params_token,
        },
    )(i)
}

fn struct_type(i: TokenList) -> IResult<TokenList, StructType> {
    map(
        pair(terminal(TerminalToken::Struct), struct_declaration),
        |(struct_token, declaration)| StructType {
            declaration,

            struct_token,
        },
    )(i)
}

fn base_type<'s>(
    can_end_line: bool,
) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, Type<'s>> {
    alt((
        map(cond_not_line_ending(!can_end_line, local_type), Type::Local),
        map(cond_not_line_ending(!can_end_line, var_type), Type::Var),
        map(cond_not_line_ending(!can_end_line, plain_type), Type::Plain),
        map(void_function_ref_type, Type::FunctionRef),
        map(struct_type, Type::Struct),
    ))
}

// Postfix types
enum AlmostPostfixType<'s> {
    Array {
        len: Box<Expression<'s>>,

        open_len_token: &'s Token<'s>,
        close_len_token: &'s Token<'s>,
    },
    Generic {
        params: SeparatedList1<'s, Type<'s>>,

        open_params_token: &'s Token<'s>,
        trailing_separator_token: Option<&'s Token<'s>>,
        close_params_token: &'s Token<'s>,
    },
    FunctionRef {
        params: SeparatedList0<'s, FunctionParam<'s>>,

        functionref_token: &'s Token<'s>,
        open_params_token: &'s Token<'s>,
        trailing_separator_token: Option<&'s Token<'s>>,
        close_params_token: &'s Token<'s>,
    },
    Reference {
        ref_token: &'s Token<'s>,
    },
    Nullable {
        nullable_token: &'s Token<'s>,
    },
}

impl<'s> AlmostPostfixType<'s> {
    fn into_type(self, base: Type<'s>) -> Type<'s> {
        match self {
            AlmostPostfixType::Array {
                len,
                open_len_token,
                close_len_token,
            } => Type::Array(ArrayType {
                base: Box::new(base),
                len,

                open_len_token,
                close_len_token,
            }),
            AlmostPostfixType::Generic {
                params,
                open_params_token,
                trailing_separator_token,
                close_params_token,
            } => Type::Generic(GenericType {
                base: Box::new(base),
                params,

                open_params_token,
                trailing_separator_token,
                close_params_token,
            }),
            AlmostPostfixType::FunctionRef {
                params,
                functionref_token,
                open_params_token,
                trailing_separator_token,
                close_params_token,
            } => Type::FunctionRef(FunctionRefType {
                ret_type: Some(Box::new(base)),
                params,

                functionref_token,
                open_params_token,
                trailing_separator_token,
                close_params_token,
            }),
            AlmostPostfixType::Reference { ref_token } => Type::Reference(ReferenceType {
                base: Box::new(base),

                ref_token,
            }),
            AlmostPostfixType::Nullable { nullable_token } => Type::Nullable(NullableType {
                base: Box::new(base),

                nullable_token,
            }),
        }
    }
}

fn array_type(i: TokenList) -> IResult<TokenList, AlmostPostfixType> {
    map(
        pair(
            terminal(TerminalToken::OpenSquare),
            perfect(
                "array type",
                pair(expression, terminal(TerminalToken::CloseSquare)),
            ),
        ),
        |(open_len_token, (len, close_len_token))| AlmostPostfixType::Array {
            len: Box::new(len),

            open_len_token,
            close_len_token,
        },
    )(i)
}

fn generic_type(i: TokenList) -> IResult<TokenList, AlmostPostfixType> {
    map(
        tuple((
            terminal(TerminalToken::Less),
            tuple((
                separated_list1(TerminalToken::Comma, ty(true)),
                opt(terminal(TerminalToken::Comma)),
                terminal(TerminalToken::Greater),
            )),
        )),
        |(open_params_token, (params, trailing_separator_token, close_params_token))| {
            AlmostPostfixType::Generic {
                params,

                open_params_token,
                trailing_separator_token,
                close_params_token,
            }
        },
    )(i)
}

fn function_ref_type(i: TokenList) -> IResult<TokenList, AlmostPostfixType> {
    map(
        pair(
            terminal(TerminalToken::FunctionRef),
            perfect(
                "functionref type",
                tuple((
                    terminal(TerminalToken::OpenBracket),
                    separated_list0(TerminalToken::Comma, function_param),
                    opt(terminal(TerminalToken::Comma)),
                    terminal(TerminalToken::CloseBracket),
                )),
            ),
        ),
        |(
            functionref_token,
            (open_params_token, params, trailing_separator_token, close_params_token),
        )| AlmostPostfixType::FunctionRef {
            params,

            functionref_token,
            open_params_token,
            trailing_separator_token,
            close_params_token,
        },
    )(i)
}

fn reference_type(i: TokenList) -> IResult<TokenList, AlmostPostfixType> {
    map(terminal(TerminalToken::BitwiseAnd), |ref_token| {
        AlmostPostfixType::Reference { ref_token }
    })(i)
}

fn nullable_type(i: TokenList) -> IResult<TokenList, AlmostPostfixType> {
    map(terminal(TerminalToken::OrNull), |nullable_token| {
        AlmostPostfixType::Nullable { nullable_token }
    })(i)
}
