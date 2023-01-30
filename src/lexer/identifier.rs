use crate::lexer::parse_str::ParseStr;
use crate::token::{TerminalToken, TokenType};
use crate::Flavor;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub fn try_identifier(val: ParseStr, flavor: Flavor) -> Option<(TokenType, ParseStr)> {
    let (identifier_str, remaining) = try_identifier_str(val)?;
    let token_ty = match identifier_as_token(identifier_str, flavor) {
        Some(terminal) => TokenType::Terminal(terminal),
        None => TokenType::Identifier(identifier_str),
    };
    Some((token_ty, remaining))
}

fn is_identifier_char(c: char) -> bool {
    c == '_' || c.is_ascii_alphanumeric()
}

fn try_identifier_str(val: ParseStr) -> Option<(&str, ParseStr)> {
    let first_char = val.as_str().chars().next()?;
    if first_char != '_' && !first_char.is_ascii_alphabetic() {
        return None;
    }

    Some(val.split_at(val.as_str().find(|c: char| !is_identifier_char(c))))
}

fn identifier_as_token(identifier: &str, flavor: Flavor) -> Option<TerminalToken> {
    lazy_static! {
        static ref IDENTIFIERS_MAP: HashMap<&'static str, TerminalToken> =
            TerminalToken::IDENTIFIERS
                .iter()
                .map(|(token, token_val)| (*token_val, *token))
                .collect();
    }

    IDENTIFIERS_MAP
        .get(identifier)
        .cloned()
        .filter(|token| token.is_supported(flavor))
}
