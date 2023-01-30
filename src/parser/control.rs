use crate::ast::{
    ForDeclaration, ForeachIndex, Identifier, IfElse, Precedence, SwitchCase, SwitchCaseCondition,
    Type,
};
use crate::parser::combinator::{alt, alternative, prevent_ending_line};
use crate::parser::expression::expression;
use crate::parser::identifier::identifier;
use crate::parser::list::many;
use crate::parser::statement::{statement, var_declaration_statement};
use crate::parser::token::terminal;
use crate::parser::type_::type_;
use crate::parser::{ContextType, ParseResult, TokenList};
use crate::token::{TerminalToken, Token};

pub fn if_else(tokens: TokenList) -> ParseResult<IfElse> {
    alternative(
        tokens,
        ContextType::ElseStatement,
        |tokens| terminal(tokens, TerminalToken::Else),
        |tokens, else_| {
            let (tokens, body) = statement(tokens)?;
            Ok((
                tokens,
                IfElse {
                    else_,
                    body: Box::new(body),
                },
            ))
        },
    )
}

pub fn switch_case(tokens: TokenList) -> ParseResult<SwitchCase> {
    let (tokens, condition) = switch_case_condition(tokens)?;
    let (tokens, colon) = terminal(tokens, TerminalToken::Colon)?;
    let (tokens, body) = many(tokens, statement)?;
    Ok((
        tokens,
        SwitchCase {
            condition,
            colon,
            body,
        },
    ))
}

pub fn switch_case_condition(tokens: TokenList) -> ParseResult<SwitchCaseCondition> {
    if let Ok((tokens, default)) = terminal(tokens, TerminalToken::Default) {
        return Ok((tokens, SwitchCaseCondition::Default { default }));
    }

    alternative(
        tokens,
        ContextType::SwitchCaseCondition,
        |tokens| terminal(tokens, TerminalToken::Case),
        |tokens, case| {
            let (tokens, value) = expression(tokens, Precedence::None)?;
            Ok((
                tokens,
                SwitchCaseCondition::Case {
                    case,
                    value: Box::new(value),
                },
            ))
        },
    )
}

pub fn for_declaration(tokens: TokenList) -> ParseResult<ForDeclaration> {
    // Weird hack:
    // We need to try parsing a VarDeclarationStatement first since otherwise the Expression can
    // falsely fail too early by parsing a type as an Expression::Var. However we want to propagate
    // errors from the declaration statement if both fail, since a type syntax error is more useful
    // there.
    let declaration_err = match var_declaration_statement(tokens) {
        Ok((tokens, declaration)) => return Ok((tokens, ForDeclaration::Declaration(declaration))),
        Err(err) if err.is_fatal => return Err(err),
        Err(err) => err,
    };

    match expression(tokens, Precedence::None) {
        Ok((tokens, expression)) => Ok((tokens, ForDeclaration::Expression(Box::new(expression)))),
        Err(err) if err.is_fatal => Err(err),

        // Swap in the var declaration's error.
        Err(_) => Err(declaration_err),
    }
}

pub fn foreach_index(tokens: TokenList) -> ParseResult<ForeachIndex> {
    alt(typed_foreach_index(tokens)).unwrap_or_else(|| untyped_foreach_index(tokens))
}

fn typed_foreach_index(tokens: TokenList) -> ParseResult<ForeachIndex> {
    let (tokens, ty) =
        prevent_ending_line(tokens, type_(tokens).map_err(|err| err.into_non_fatal()))?;
    let (tokens, name) = identifier(tokens)?;
    let (tokens, comma) = terminal(tokens, TerminalToken::Comma)?;
    Ok((
        tokens,
        ForeachIndex {
            ty: Some(ty),
            name,
            comma,
        },
    ))
}

fn untyped_foreach_index(tokens: TokenList) -> ParseResult<ForeachIndex> {
    let (tokens, name) = identifier(tokens)?;
    let (tokens, comma) = terminal(tokens, TerminalToken::Comma)?;
    Ok((
        tokens,
        ForeachIndex {
            ty: None,
            name,
            comma,
        },
    ))
}

pub fn foreach_value(tokens: TokenList) -> ParseResult<(Option<Type>, Identifier, &Token)> {
    alt(typed_foreach_value(tokens)).unwrap_or_else(|| untyped_foreach_value(tokens))
}

fn typed_foreach_value(tokens: TokenList) -> ParseResult<(Option<Type>, Identifier, &Token)> {
    let (tokens, ty) =
        prevent_ending_line(tokens, type_(tokens).map_err(|err| err.into_non_fatal()))?;
    let (tokens, name) = identifier(tokens)?;
    let (tokens, in_) = terminal(tokens, TerminalToken::In)?;
    Ok((tokens, (Some(ty), name, in_)))
}

fn untyped_foreach_value(tokens: TokenList) -> ParseResult<(Option<Type>, Identifier, &Token)> {
    let (tokens, name) = identifier(tokens)?;
    let (tokens, in_) = terminal(tokens, TerminalToken::In)?;
    Ok((tokens, (None, name, in_)))
}
