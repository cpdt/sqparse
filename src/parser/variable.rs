use crate::ast::{Precedence, VarDeclaration, VarInitializer};
use crate::parser::combinator::{alternative, opt};
use crate::parser::expression::expression;
use crate::parser::identifier::identifier;
use crate::parser::token::terminal;
use crate::parser::{ContextType, ParseResult, TokenList};
use crate::token::TerminalToken;

pub fn var_declaration(tokens: TokenList) -> ParseResult<VarDeclaration> {
    let (tokens, name) = identifier(tokens)?;
    let (tokens, initializer) = opt(tokens, var_initializer(tokens))?;

    Ok((tokens, VarDeclaration { name, initializer }))
}

pub fn var_initializer(tokens: TokenList) -> ParseResult<VarInitializer> {
    alternative(
        tokens,
        ContextType::VarInitializer,
        |tokens| terminal(tokens, TerminalToken::Assign),
        |tokens, assign| {
            let (tokens, value) = expression(tokens, Precedence::Comma)?;
            Ok((
                tokens,
                VarInitializer {
                    assign,
                    value: Box::new(value),
                },
            ))
        },
    )
}
