use crate::ast::{Identifier, MethodIdentifier};
use crate::parser::combinator::{alt_map, map};
use crate::parser::token::terminal;
use crate::parser::token_list::TokenList;
use crate::parser::ParseResult;
use crate::token::{TerminalToken, TokenType};
use crate::{ParseError, ParseErrorType};

pub fn identifier(tokens: TokenList) -> ParseResult<Identifier> {
    if let Some((tokens, item)) = tokens.split_first() {
        if let TokenType::Identifier(value) = item.token.ty {
            return Ok((
                tokens,
                Identifier {
                    value,
                    token: &item.token,
                },
            ));
        }
    }

    Err(ParseError::new(
        ParseErrorType::ExpectedIdentifier,
        tokens.start_index(),
    ))
}

pub fn method_identifier(tokens: TokenList) -> ParseResult<MethodIdentifier> {
    alt_map(
        terminal(tokens, TerminalToken::Constructor),
        MethodIdentifier::Constructor,
    )
    .unwrap_or_else(|| map(identifier(tokens), MethodIdentifier::Identifier))
}
