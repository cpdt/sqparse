use crate::ast::{Identifier, MethodIdentifier};
use crate::parser::parse_result_ext::ParseResultExt;
use crate::parser::token_list::TokenList;
use crate::parser::token_list_ext::TokenListExt;
use crate::parser::ParseResult;
use crate::token::{TerminalToken, TokenType};
use crate::ParseErrorType;

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

    Err(tokens.error(ParseErrorType::ExpectedIdentifier))
}

pub fn method_identifier(tokens: TokenList) -> ParseResult<MethodIdentifier> {
    tokens
        .terminal(TerminalToken::Constructor)
        .map_val(MethodIdentifier::Constructor)
        .or_try(|| identifier(tokens).map_val(MethodIdentifier::Identifier))
}
