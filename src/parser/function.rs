use crate::ast::{
    FunctionArg, FunctionArgs, FunctionCaptures, FunctionDeclaration, FunctionEnvironment,
    FunctionRefArg, Precedence, SeparatedList1,
};
use crate::parser::combinator::{alt, definitely, opt, span};
use crate::parser::expression::expression;
use crate::parser::identifier::identifier;
use crate::parser::list::separated_list_trailing0;
use crate::parser::statement::statement;
use crate::parser::token::terminal;
use crate::parser::type_::type_;
use crate::parser::variable::var_initializer;
use crate::parser::{ContextType, ParseResult, TokenList};
use crate::token::TerminalToken;

pub fn function_declaration(tokens: TokenList) -> ParseResult<FunctionDeclaration> {
    let (tokens, environment) = opt(tokens, function_environment(tokens))?;
    let (tokens, (open, args, close)) = span(
        tokens,
        ContextType::FunctionDeclarationArgs,
        TerminalToken::OpenBracket,
        TerminalToken::CloseBracket,
        |tokens, open, close| {
            let (tokens, args) = function_args(tokens)?;
            Ok((tokens, (open, args, close)))
        },
    )?;
    let (tokens, captures) = opt(tokens, function_captures(tokens))?;
    let (tokens, body) = statement(tokens)?;

    Ok((
        tokens,
        FunctionDeclaration {
            environment,
            open,
            args,
            close,
            captures,
            body: Box::new(body),
        },
    ))
}

pub fn function_environment(tokens: TokenList) -> ParseResult<FunctionEnvironment> {
    span(
        tokens,
        ContextType::FunctionDeclarationEnvironment,
        TerminalToken::OpenSquare,
        TerminalToken::CloseSquare,
        |tokens, open, close| {
            let (tokens, value) = expression(tokens, Precedence::None)?;
            Ok((
                tokens,
                FunctionEnvironment {
                    open,
                    value: Box::new(value),
                    close,
                },
            ))
        },
    )
}

pub fn function_args(tokens: TokenList) -> ParseResult<FunctionArgs> {
    let (tokens, maybe_list) = separated_list_trailing0(tokens, function_arg, |tokens| {
        terminal(tokens, TerminalToken::Comma)
    })?;
    let Some(list) = maybe_list else {
        // If there are no arguments, the function can still be variable without requiring a
        // trailing comma.
        return match terminal(tokens, TerminalToken::Ellipsis) {
            Ok((tokens, vararg)) => Ok((tokens, FunctionArgs::EmptyVariable { vararg })),
            Err(_) => Ok((tokens, FunctionArgs::NonVariable { args: None }))
        };
    };

    // If the list has a trailing comma, it can also have a vararg.
    if let Some(comma) = list.trailing {
        let (tokens, vararg) = opt(tokens, terminal(tokens, TerminalToken::Ellipsis))?;
        if let Some(vararg) = vararg {
            return Ok((
                tokens,
                FunctionArgs::NonEmptyVariable {
                    args: SeparatedList1 {
                        items: list.items,
                        last_item: list.last_item,
                    },
                    comma,
                    vararg,
                },
            ));
        }
    }

    Ok((tokens, FunctionArgs::NonVariable { args: Some(list) }))
}

pub fn function_arg(tokens: TokenList) -> ParseResult<FunctionArg> {
    alt(typed_function_arg(tokens)).unwrap_or_else(|| untyped_function_arg(tokens))
}

fn typed_function_arg(tokens: TokenList) -> ParseResult<FunctionArg> {
    let (tokens, ty) = type_(tokens)?;
    let (tokens, name) = identifier(tokens)?;
    let (tokens, initializer) = opt(tokens, var_initializer(tokens))?;

    Ok((
        tokens,
        FunctionArg {
            ty: Some(ty),
            name,
            initializer,
        },
    ))
}

fn untyped_function_arg(tokens: TokenList) -> ParseResult<FunctionArg> {
    let (tokens, name) = identifier(tokens)?;
    let (tokens, initializer) = opt(tokens, var_initializer(tokens))?;

    Ok((
        tokens,
        FunctionArg {
            ty: None,
            name,
            initializer,
        },
    ))
}

pub fn function_captures(tokens: TokenList) -> ParseResult<FunctionCaptures> {
    definitely(
        tokens,
        ContextType::FunctionDeclarationCaptures,
        |tokens| terminal(tokens, TerminalToken::Colon),
        |tokens, colon| {
            span(
                tokens,
                ContextType::FunctionDeclarationCaptures,
                TerminalToken::OpenBracket,
                TerminalToken::CloseBracket,
                |tokens, open, close| {
                    let (tokens, names) = separated_list_trailing0(tokens, identifier, |tokens| {
                        terminal(tokens, TerminalToken::Comma)
                    })?;
                    Ok((
                        tokens,
                        FunctionCaptures {
                            colon,
                            open,
                            names,
                            close,
                        },
                    ))
                },
            )
        },
    )
}

pub fn function_ref_arg(tokens: TokenList) -> ParseResult<FunctionRefArg> {
    let (tokens, ty) = type_(tokens)?;
    let (tokens, name) = opt(tokens, identifier(tokens))?;
    let (tokens, initializer) = opt(tokens, var_initializer(tokens))?;

    Ok((
        tokens,
        FunctionRefArg {
            ty,
            name,
            initializer,
        },
    ))
}
