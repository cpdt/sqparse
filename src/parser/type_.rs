use crate::ast::{
    ArrayType, FunctionRefType, GenericType, LocalType, NullableType, PlainType, Precedence,
    ReferenceType, StructType, Type, VarType,
};
use crate::parser::combinator::{alt_map, alternative, first_of, map, span};
use crate::parser::expression::expression;
use crate::parser::function::function_ref_arg;
use crate::parser::identifier::identifier;
use crate::parser::list::{separated_list_trailing0, separated_list_trailing1};
use crate::parser::struct_::struct_declaration;
use crate::parser::token::terminal;
use crate::parser::{ContextType, ParseError, ParseErrorType, ParseResult, TokenList};
use crate::token::TerminalToken;

pub fn type_(tokens: TokenList) -> ParseResult<Type> {
    let (mut tokens, mut type_) = base_type(tokens)?;

    loop {
        let mut type_container = Some(type_);
        match modifier_type(tokens, TypeRef(&mut type_container)) {
            Ok((new_tokens, new_type)) => {
                tokens = new_tokens;
                type_ = new_type;
            }
            Err(err) if err.is_fatal => return Err(err),
            Err(_) => return Ok((tokens, type_container.unwrap())),
        }
    }
}

fn base_type(tokens: TokenList) -> ParseResult<Type> {
    first_of(
        tokens,
        [
            |tokens| map(local_type(tokens), Type::Local),
            |tokens| map(var_type(tokens), Type::Var),
            |tokens| map(plain_type(tokens), Type::Plain),
            |tokens| map(void_function_ref_type(tokens), Type::FunctionRef),
            |tokens| map(struct_type(tokens), Type::Struct),
        ],
        |_| {
            Err(ParseError::new(
                ParseErrorType::ExpectedType,
                tokens.start_index(),
            ))
        },
    )
}

pub fn local_type(tokens: TokenList) -> ParseResult<LocalType> {
    let (tokens, local) = terminal(tokens, TerminalToken::Local)?;
    Ok((tokens, LocalType { local }))
}

pub fn var_type(tokens: TokenList) -> ParseResult<VarType> {
    let (tokens, var) = terminal(tokens, TerminalToken::Var)?;
    Ok((tokens, VarType { var }))
}

pub fn plain_type(tokens: TokenList) -> ParseResult<PlainType> {
    let (tokens, name) = identifier(tokens)?;
    Ok((tokens, PlainType { name }))
}

pub fn void_function_ref_type(tokens: TokenList) -> ParseResult<FunctionRefType> {
    function_ref_type(tokens, || None)
}

pub fn struct_type(tokens: TokenList) -> ParseResult<StructType> {
    alternative(
        tokens,
        ContextType::StructType,
        |tokens| terminal(tokens, TerminalToken::Struct),
        |tokens, struct_| {
            let (tokens, declaration) = struct_declaration(tokens)?;
            Ok((
                tokens,
                StructType {
                    struct_,
                    declaration,
                },
            ))
        },
    )
}

struct TypeRef<'a, 's>(&'a mut Option<Type<'s>>);
impl<'a, 's> TypeRef<'a, 's> {
    fn take(self) -> Type<'s> {
        self.0.take().unwrap()
    }
}

fn modifier_type<'s>(tokens: TokenList<'s>, left: TypeRef<'_, 's>) -> ParseResult<'s, Type<'s>> {
    let left_ref = left.0;
    alt_map(array_type(tokens, TypeRef(left_ref)), Type::Array)
        .or_else(|| alt_map(generic_type(tokens, TypeRef(left_ref)), Type::Generic))
        .or_else(|| {
            alt_map(
                nonvoid_function_ref_type(tokens, TypeRef(left_ref)),
                Type::FunctionRef,
            )
        })
        .or_else(|| alt_map(reference_type(tokens, TypeRef(left_ref)), Type::Reference))
        .or_else(|| alt_map(nullable_type(tokens, TypeRef(left_ref)), Type::Nullable))
        .unwrap_or_else(|| {
            Err(ParseError::new(
                ParseErrorType::ExpectedTypeModifier,
                tokens.start_index(),
            ))
        })
}

fn array_type<'s>(tokens: TokenList<'s>, left: TypeRef<'_, 's>) -> ParseResult<'s, ArrayType<'s>> {
    span(
        tokens,
        ContextType::ArrayType,
        TerminalToken::OpenSquare,
        TerminalToken::CloseSquare,
        |tokens, open, close| {
            let (tokens, len) = expression(tokens, Precedence::None)?;
            Ok((
                tokens,
                ArrayType {
                    base: Box::new(left.take()),
                    open,
                    len: Box::new(len),
                    close,
                },
            ))
        },
    )
}

fn generic_type<'s>(
    tokens: TokenList<'s>,
    left: TypeRef<'_, 's>,
) -> ParseResult<'s, GenericType<'s>> {
    alternative(
        tokens,
        ContextType::GenericType,
        |tokens| terminal(tokens, TerminalToken::Less),
        |tokens, open| {
            let (tokens, params) = separated_list_trailing1(tokens, type_, |tokens| {
                terminal(tokens, TerminalToken::Comma)
            })?;
            let (tokens, close) = terminal(tokens, TerminalToken::Greater)?;

            Ok((
                tokens,
                GenericType {
                    base: Box::new(left.take()),
                    open,
                    params,
                    close,
                },
            ))
        },
    )
}

fn nonvoid_function_ref_type<'s>(
    tokens: TokenList<'s>,
    left: TypeRef<'_, 's>,
) -> ParseResult<'s, FunctionRefType<'s>> {
    function_ref_type(tokens, || Some(left.take()))
}

fn reference_type<'s>(
    tokens: TokenList<'s>,
    left: TypeRef<'_, 's>,
) -> ParseResult<'s, ReferenceType<'s>> {
    let (tokens, reference) = terminal(tokens, TerminalToken::BitwiseAnd)?;
    Ok((
        tokens,
        ReferenceType {
            base: Box::new(left.take()),
            reference,
        },
    ))
}

fn nullable_type<'s>(
    tokens: TokenList<'s>,
    left: TypeRef<'_, 's>,
) -> ParseResult<'s, NullableType<'s>> {
    let (tokens, ornull) = terminal(tokens, TerminalToken::OrNull)?;
    Ok((
        tokens,
        NullableType {
            base: Box::new(left.take()),
            ornull,
        },
    ))
}

fn function_ref_type<'s>(
    tokens: TokenList<'s>,
    return_type: impl FnOnce() -> Option<Type<'s>>,
) -> ParseResult<'s, FunctionRefType<'s>> {
    alternative(
        tokens,
        ContextType::FunctionRefType,
        |tokens| terminal(tokens, TerminalToken::FunctionRef),
        |tokens, functionref| {
            span(
                tokens,
                ContextType::FunctionRefType,
                TerminalToken::OpenBracket,
                TerminalToken::CloseBracket,
                |tokens, open, close| {
                    let (tokens, args) =
                        separated_list_trailing0(tokens, function_ref_arg, |tokens| {
                            terminal(tokens, TerminalToken::Comma)
                        })?;
                    Ok((
                        tokens,
                        FunctionRefType {
                            return_type: return_type().map(Box::new),
                            functionref,
                            open,
                            args,
                            close,
                        },
                    ))
                },
            )
        },
    )
}
