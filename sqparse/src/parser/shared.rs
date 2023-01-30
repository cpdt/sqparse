use crate::ast::{
    ClassDeclaration, ClassExtends, ClassMember, ClassMemberType, FunctionArg, FunctionArgs,
    FunctionArgsTrailing, FunctionCaptures, FunctionDeclaration, FunctionEnvironment, Identifier,
    LiteralExpression, SeparatedList0, SeparatedList1, StructDeclaration, StructInitializer,
    StructProperty, VarInitializer,
};
use crate::parse_error::ErrorType;
use crate::parser::expression::{expression, non_comma_expression, table_expression_delimited};
use crate::parser::statement::inline_statement;
use crate::parser::ty::ty;
use crate::parser::TokenList;
use crate::token::{TerminalToken, TokenType};
use crate::{IResult, ParseError, Token};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{consumed, map, opt, verify};
use nom::error::context;
use nom::multi::fold_many0;
use nom::sequence::{pair, tuple};
use nom::{Err, InputTake, Needed};
use std::fmt::Debug;
use std::num::NonZeroUsize;

pub fn perfect<'s, O>(
    ctx: &'static str,
    mut f: impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, O>,
) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, O> {
    move |i| {
        context(ctx, |i| f(i))(i).map_err(|err| match err {
            Err::Error(err) => Err::Failure(err),
            _ => err,
        })
    }
}

pub fn first_matching<'s>(
    token_type: TokenType<'s>,
) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, &'s Token<'s>> {
    map(tag(token_type), TokenList::always_first)
}

pub fn terminal<'s>(
    terminal: TerminalToken,
) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, &'s Token<'s>> {
    move |i| match i.first() {
        Some(
            token
            @
            Token {
                ty: TokenType::Terminal(received_ty),
                ..
            },
        ) if *received_ty == terminal => {
            let (remaining, _) = i.take_split(1);
            Ok((remaining, token))
        }
        Some(_) => Err(Err::Error(ParseError::new(
            i.get_offset(),
            ErrorType::ExpectedTerminal(terminal),
        ))),
        None => Err(Err::Incomplete(Needed::Size(NonZeroUsize::new(1).unwrap()))),
    }
}

pub fn identifier(tokens: TokenList) -> IResult<TokenList, Identifier> {
    match tokens.first() {
        Some(
            token
            @
            Token {
                ty: TokenType::Identifier(id),
                ..
            },
        ) => {
            let (remaining, _) = tokens.take_split(1);
            Ok((remaining, Identifier { value: *id, token }))
        }
        _ => Err(Err::Error(ParseError::new(
            tokens.offset,
            ErrorType::ExpectedIdentifier,
        ))),
    }
}

pub fn literal(tokens: TokenList) -> IResult<TokenList, LiteralExpression> {
    match tokens.first() {
        Some(
            token
            @
            Token {
                ty: TokenType::Literal(literal),
                ..
            },
        ) => {
            let (remaining, _) = tokens.take_split(1);
            Ok((
                remaining,
                LiteralExpression {
                    literal: *literal,
                    token,
                },
            ))
        }
        _ => Err(Err::Error(ParseError::new(
            tokens.offset,
            ErrorType::ExpectedLiteral,
        ))),
    }
}

pub fn adjacent_2<'s>(
    terminal_a: TerminalToken,
    terminal_b: TerminalToken,
) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, (&'s Token<'s>, &'s Token<'s>)> {
    move |i| {
        let (new_i, (token_a, token_b)) = pair(terminal(terminal_a), terminal(terminal_b))(i)?;
        if token_a.range.end != token_b.range.start {
            return Err(Err::Error(ParseError::new(
                i.offset,
                ErrorType::ExpectedCompound2(terminal_a, terminal_b),
            )));
        }
        Ok((new_i, (token_a, token_b)))
    }
}

pub fn adjacent_3<'s>(
    terminal_a: TerminalToken,
    terminal_b: TerminalToken,
    terminal_c: TerminalToken,
) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, (&'s Token<'s>, &'s Token<'s>, &'s Token<'s>)>
{
    move |i| {
        let (new_i, (token_a, token_b, token_c)) = tuple((
            terminal(terminal_a),
            terminal(terminal_b),
            terminal(terminal_c),
        ))(i)?;
        if token_a.range.end != token_b.range.start || token_b.range.end != token_c.range.start {
            return Err(Err::Error(ParseError::new(
                i.offset,
                ErrorType::ExpectedCompound3(terminal_a, terminal_b, terminal_c),
            )));
        }
        Ok((new_i, (token_a, token_b, token_c)))
    }
}

pub fn is_end_of_line<'s, O, F: FnMut(TokenList<'s>) -> IResult<TokenList<'s>, O>>(
    f: F,
) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, (bool, O)> {
    map(consumed(f), |(tokens, val)| (tokens.end_of_line(), val))
}

pub fn not_line_ending<'s, O, F: FnMut(TokenList<'s>) -> IResult<TokenList<'s>, O>>(
    f: F,
) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, O> {
    map(
        verify(consumed(f), |(tokens, _)| !tokens.end_of_line()),
        |(_, out)| out,
    )
}

pub fn cond_not_line_ending<'s, O, F: FnMut(TokenList<'s>) -> IResult<TokenList<'s>, O>>(
    cond: bool,
    f: F,
) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, O> {
    map(
        verify(consumed(f), move |(tokens, _)| {
            !cond || !tokens.end_of_line()
        }),
        |(_, out)| out,
    )
}

// Each parser can be separated by a newline or terminator, but neither is required for the last
// parser
pub fn many_separated<'s, O1, O2, O3, F, S, M>(
    mut value_parser: F,
    mut separator_parser: S,
    mut mapper: M,
) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, Vec<O3>>
where
    F: FnMut(TokenList<'s>) -> IResult<TokenList<'s>, O1>,
    S: FnMut(TokenList<'s>) -> IResult<TokenList<'s>, O2>,
    M: FnMut(O1, Option<O2>) -> O3,
{
    move |mut tokens| {
        let mut values = Vec::new();

        loop {
            let (i, (consumed_input, value)) = match consumed(|i| value_parser(i))(tokens) {
                Ok(res) => res,
                Err(Err::Error(_)) => break,
                Err(err) => return Err(err),
            };
            tokens = i;

            if tokens.is_empty() || consumed_input.end_of_line() {
                values.push(mapper(value, None));

                if tokens.is_empty() {
                    break;
                }

                continue;
            }

            match separator_parser(tokens) {
                Ok((i, separator_token)) => {
                    values.push(mapper(value, Some(separator_token)));
                    tokens = i;
                }
                Err(Err::Error(_)) => {
                    values.push(mapper(value, None));
                    break;
                }
                Err(err) => return Err(err),
            }
        }

        Ok((tokens, values))
    }
}

pub fn many_separated_till<'s, O1, O2, O3, O4, F, S, T, M>(
    mut value_parser: F,
    mut separator_parser: S,
    mut terminator_parser: T,
    mut mapper: M,
) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, (Vec<O4>, O3)>
where
    F: FnMut(TokenList<'s>) -> IResult<TokenList<'s>, O1>,
    S: FnMut(TokenList<'s>) -> IResult<TokenList<'s>, O2>,
    T: FnMut(TokenList<'s>) -> IResult<TokenList<'s>, O3>,
    M: FnMut(O1, Option<O2>) -> O4,
{
    move |mut tokens| {
        let mut values = Vec::new();

        loop {
            match terminator_parser(tokens) {
                Ok((i, end_token)) => return Ok((i, (values, end_token))),
                Err(Err::Error(_)) => {}
                Err(err) => return Err(err),
            }

            let (i, (consumed_input, value)) = consumed(|i| value_parser(i))(tokens)?;
            tokens = i;

            if consumed_input.end_of_line() {
                values.push(mapper(value, None));
                continue;
            }

            match terminator_parser(tokens) {
                Ok((i, end_token)) => {
                    values.push(mapper(value, None));
                    return Ok((i, (values, end_token)));
                }
                Err(Err::Error(_)) => {}
                Err(err) => return Err(err),
            }

            let (i, separator_token) = separator_parser(tokens)?;
            values.push(mapper(value, Some(separator_token)));
            tokens = i;
        }
    }
}

// Same as nom's separated_list0, but includes the separator token
pub fn separated_list1<'s, O, F>(
    separator: TerminalToken,
    mut f: F,
) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, SeparatedList1<'s, O>>
where
    O: Debug + Clone,
    F: FnMut(TokenList<'s>) -> IResult<TokenList<'s>, O>,
{
    move |i| {
        let (i, first_val) = f(i)?;
        let list = SeparatedList1 {
            items: Vec::new(),
            last_item: Box::new(first_val),
        };
        fold_many0(
            pair(terminal(separator), |i| f(i)),
            move || list.clone(),
            |list, (separator, item)| list.push(separator, item),
        )(i)
    }
}

pub fn separated_list0<'s, O, F>(
    separator: TerminalToken,
    f: F,
) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, SeparatedList0<'s, O>>
where
    O: Debug + Clone,
    F: FnMut(TokenList<'s>) -> IResult<TokenList<'s>, O>,
{
    opt(separated_list1(separator, f))
}

pub fn var_initializer(i: TokenList) -> IResult<TokenList, VarInitializer> {
    map(
        pair(
            terminal(TerminalToken::Assign),
            perfect("initializer", non_comma_expression),
        ),
        |(separator_token, value)| VarInitializer {
            value: Box::new(value),
            separator_token,
        },
    )(i)
}

fn function_environment(i: TokenList) -> IResult<TokenList, FunctionEnvironment> {
    map(
        tuple((
            terminal(TerminalToken::OpenSquare),
            perfect("function environment", expression),
            terminal(TerminalToken::CloseSquare),
        )),
        |(open_token, environment, close_token)| FunctionEnvironment {
            environment: Box::new(environment),

            open_token,
            close_token,
        },
    )(i)
}

fn function_arg(i: TokenList) -> IResult<TokenList, FunctionArg> {
    alt((
        map(
            tuple((ty(true), identifier, opt(var_initializer))),
            |(arg_type, arg_name, initializer)| FunctionArg {
                arg_type: Some(arg_type),
                arg_name,
                initializer,
            },
        ),
        map(
            pair(identifier, opt(var_initializer)),
            |(arg_name, initializer)| FunctionArg {
                arg_type: None,
                arg_name,
                initializer,
            },
        ),
    ))(i)
}

fn function_args_trailing(i: TokenList) -> IResult<TokenList, FunctionArgsTrailing> {
    map(
        pair(
            terminal(TerminalToken::Comma),
            opt(terminal(TerminalToken::Ellipsis)),
        ),
        |(separator_token, maybe_vararg_token)| match maybe_vararg_token {
            Some(vararg_token) => FunctionArgsTrailing::VarArg {
                separator_token,
                vararg_token,
            },
            None => FunctionArgsTrailing::Separator {
                token: separator_token,
            },
        },
    )(i)
}

fn function_arg_list(i: TokenList) -> IResult<TokenList, FunctionArgs> {
    let non_empty_parser = map(
        pair(
            separated_list1(TerminalToken::Comma, function_arg),
            opt(function_args_trailing),
        ),
        |(arg_list, trailing)| FunctionArgs::List { arg_list, trailing },
    );
    let empty_parser = map(opt(terminal(TerminalToken::Ellipsis)), |vararg_token| {
        FunctionArgs::Empty { vararg_token }
    });
    alt((non_empty_parser, empty_parser))(i)
}

fn function_captures(i: TokenList) -> IResult<TokenList, FunctionCaptures> {
    map(
        tuple((
            terminal(TerminalToken::Colon),
            terminal(TerminalToken::OpenBracket),
            separated_list0(TerminalToken::Comma, identifier),
            opt(terminal(TerminalToken::Comma)),
            terminal(TerminalToken::CloseBracket),
        )),
        |(separator_token, open_token, capture_names, trailing_separator_token, close_token)| {
            FunctionCaptures {
                capture_names,

                separator_token,
                open_token,
                trailing_separator_token,
                close_token,
            }
        },
    )(i)
}

pub fn function_declaration(i: TokenList) -> IResult<TokenList, FunctionDeclaration> {
    map(
        tuple((
            opt(function_environment),
            terminal(TerminalToken::OpenBracket),
            perfect("function args", function_arg_list),
            terminal(TerminalToken::CloseBracket),
            opt(function_captures),
            perfect("function body", inline_statement),
        )),
        |(environment, open_args_token, args, close_args_token, captures, body)| {
            FunctionDeclaration {
                environment,
                args,
                captures,
                body: Box::new(body),

                open_args_token,
                close_args_token,
            }
        },
    )(i)
}

fn class_extends(i: TokenList) -> IResult<TokenList, ClassExtends> {
    map(
        pair(terminal(TerminalToken::Extends), expression),
        |(extends_token, name)| ClassExtends {
            name: Box::new(name),

            extends_token,
        },
    )(i)
}

fn class_member_type(i: TokenList) -> IResult<TokenList, ClassMemberType> {
    let property_member = map(
        tuple((
            opt(terminal(TerminalToken::Static)),
            identifier,
            terminal(TerminalToken::Assign),
            expression,
            opt(terminal(TerminalToken::Semicolon)),
        )),
        |(static_token, name, separator_token, value, end_token)| ClassMemberType::Property {
            name,
            value: Box::new(value),

            static_token,
            separator_token,
            end_token,
        },
    );
    let computed_property_member = map(
        tuple((
            opt(terminal(TerminalToken::Static)),
            terminal(TerminalToken::OpenSquare),
            expression,
            terminal(TerminalToken::CloseSquare),
            terminal(TerminalToken::Assign),
            expression,
            opt(terminal(TerminalToken::Semicolon)),
        )),
        |(
            static_token,
            open_name_token,
            name,
            close_name_token,
            separator_token,
            value,
            end_token,
        )| ClassMemberType::ComputedProperty {
            name: Box::new(name),
            value: Box::new(value),

            static_token,
            open_name_token,
            close_name_token,
            separator_token,
            end_token,
        },
    );
    let constructor_member = map(
        pair(terminal(TerminalToken::Constructor), function_declaration),
        |(constructor_token, function)| ClassMemberType::Constructor {
            function,

            constructor_token,
        },
    );
    let function_member = map(
        tuple((
            opt(ty(false)),
            terminal(TerminalToken::Function),
            identifier,
            function_declaration,
        )),
        |(return_type, function_token, name, function)| ClassMemberType::Function {
            return_type,
            name,
            function,

            function_token,
        },
    );

    alt((
        property_member,
        computed_property_member,
        constructor_member,
        function_member,
    ))(i)
}

pub fn class_declaration(i: TokenList) -> IResult<TokenList, ClassDeclaration> {
    map(
        tuple((
            opt(class_extends),
            terminal(TerminalToken::OpenBrace),
            perfect(
                "class declaration",
                many_separated_till(
                    pair(
                        opt(table_expression_delimited(
                            TerminalToken::OpenAttributes,
                            TerminalToken::CloseAttributes,
                        )),
                        class_member_type,
                    ),
                    terminal(TerminalToken::Semicolon),
                    terminal(TerminalToken::CloseBrace),
                    |(attributes, ty), separator_token| ClassMember {
                        attributes: attributes.map(Box::new),
                        ty,

                        separator_token,
                    },
                ),
            ),
        )),
        |(extends, open_members_token, (members, close_members_token))| ClassDeclaration {
            extends,
            members,

            open_members_token,
            close_members_token,
        },
    )(i)
}

fn struct_initializer(i: TokenList) -> IResult<TokenList, StructInitializer> {
    map(
        pair(terminal(TerminalToken::Assign), non_comma_expression),
        |(separator_token, value)| StructInitializer {
            value: Box::new(value),

            separator_token,
        },
    )(i)
}

pub fn struct_declaration(i: TokenList) -> IResult<TokenList, StructDeclaration> {
    map(
        tuple((
            terminal(TerminalToken::OpenBrace),
            perfect(
                "struct declaration",
                many_separated_till(
                    tuple((ty(false), identifier, opt(struct_initializer))),
                    terminal(TerminalToken::Comma),
                    terminal(TerminalToken::CloseBrace),
                    |(property_type, property_name, initializer), separator_token| StructProperty {
                        property_type,
                        property_name,
                        initializer,

                        separator_token,
                    },
                ),
            ),
        )),
        |(open_properties_token, (properties, close_properties_token))| StructDeclaration {
            properties,

            open_properties_token,
            close_properties_token,
        },
    )(i)
}
