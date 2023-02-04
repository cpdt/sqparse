use crate::ast::{
    FunctionCaptures, FunctionDefinition, FunctionEnvironment, FunctionParam, FunctionParams,
    FunctionRefParam, Precedence, SeparatedList1,
};
use crate::parser::expression::expression;
use crate::parser::identifier::identifier;
use crate::parser::parse_result_ext::ParseResultExt;
use crate::parser::statement::statement_type;
use crate::parser::token_list::TokenList;
use crate::parser::token_list_ext::TokenListExt;
use crate::parser::type_::type_;
use crate::parser::variable::var_initializer;
use crate::parser::ParseResult;
use crate::token::TerminalToken;
use crate::ContextType;

pub fn function_definition(tokens: TokenList) -> ParseResult<FunctionDefinition> {
    let (tokens, environment) = function_environment(tokens).maybe(tokens)?;
    let (tokens, (open, params, close)) = tokens.terminal(TerminalToken::OpenBracket).opens(
        ContextType::FunctionParamList,
        |tokens| tokens.terminal(TerminalToken::CloseBracket),
        |tokens, open, close| {
            let (tokens, params) = function_params(tokens)?;
            Ok((tokens, (open, params, close)))
        },
    )?;
    let (tokens, captures) = function_captures(tokens).maybe(tokens)?;
    let (tokens, body) = statement_type(tokens).replace_context_from(
        ContextType::BlockStatement,
        ContextType::Span,
        tokens,
    )?;

    Ok((
        tokens,
        FunctionDefinition {
            environment,
            open,
            params,
            close,
            captures,
            body: Box::new(body),
        },
    ))
}

pub fn function_environment(tokens: TokenList) -> ParseResult<FunctionEnvironment> {
    tokens.terminal(TerminalToken::OpenSquare).opens(
        ContextType::FunctionEnvironment,
        |tokens| tokens.terminal(TerminalToken::CloseSquare),
        |tokens, open, close| {
            let (tokens, value) = expression(tokens, Precedence::None)?;
            Ok((tokens, FunctionEnvironment { open, value, close }))
        },
    )
}

pub fn function_params(tokens: TokenList) -> ParseResult<FunctionParams> {
    let (tokens, maybe_list) = tokens.separated_list_trailing0(function_param, |tokens| {
        tokens.terminal(TerminalToken::Comma)
    })?;

    let Some(list) = maybe_list else {
        // There are no arguments. The function can still be variable.
        return match tokens.terminal(TerminalToken::Ellipsis) {
            Ok((tokens, vararg)) => Ok((tokens, FunctionParams::EmptyVariable { vararg })),
            Err(_) => Ok((tokens, FunctionParams::NonVariable { params: None }))
        };
    };

    // If the list has a trailing comma, it can also be variable.
    if let Some(comma) = list.trailing {
        if let Ok((tokens, vararg)) = tokens.terminal(TerminalToken::Ellipsis) {
            return Ok((
                tokens,
                FunctionParams::NonEmptyVariable {
                    params: SeparatedList1 {
                        items: list.items,
                        last_item: list.last_item,
                    },
                    comma,
                    vararg,
                },
            ));
        }
    }

    Ok((tokens, FunctionParams::NonVariable { params: Some(list) }))
}

pub fn function_param(tokens: TokenList) -> ParseResult<FunctionParam> {
    typed_function_param(tokens).or_try(|| untyped_function_param(tokens))
}

fn typed_function_param(tokens: TokenList) -> ParseResult<FunctionParam> {
    type_(tokens)
        .and_then(|(tokens, type_)| identifier(tokens).map_val(|name| (type_, name)))
        .determines(|tokens, (type_, name)| {
            let (tokens, initializer) = var_initializer(tokens).maybe(tokens)?;
            Ok((
                tokens,
                FunctionParam {
                    type_: Some(type_),
                    name,
                    initializer,
                },
            ))
        })
}

fn untyped_function_param(tokens: TokenList) -> ParseResult<FunctionParam> {
    identifier(tokens).determines(|tokens, name| {
        let (tokens, initializer) = var_initializer(tokens).maybe(tokens)?;
        Ok((
            tokens,
            FunctionParam {
                type_: None,
                name,
                initializer,
            },
        ))
    })
}

pub fn function_captures(tokens: TokenList) -> ParseResult<FunctionCaptures> {
    tokens
        .terminal(TerminalToken::Colon)
        .determines(|tokens, colon| {
            let (tokens, (open, names, close)) =
                tokens.terminal(TerminalToken::OpenBracket).opens(
                    ContextType::FunctionCaptureList,
                    |tokens| tokens.terminal(TerminalToken::CloseBracket),
                    |tokens, open, close| {
                        tokens
                            .separated_list_trailing0(identifier, |tokens| {
                                tokens.terminal(TerminalToken::Comma)
                            })
                            .map_val(|names| (open, names, close))
                    },
                )?;

            Ok((
                tokens,
                FunctionCaptures {
                    colon,
                    open,
                    names,
                    close,
                },
            ))
        })
}

pub fn function_ref_param(tokens: TokenList) -> ParseResult<FunctionRefParam> {
    let (tokens, type_) = type_(tokens)?;
    let (tokens, name) = identifier(tokens).maybe(tokens)?;
    let (tokens, initializer) = var_initializer(tokens).maybe(tokens)?;
    Ok((
        tokens,
        FunctionRefParam {
            type_,
            name,
            initializer,
        },
    ))
}
