use crate::ast::{SeparatedList1, SeparatedListTrailing0, SeparatedListTrailing1};
use crate::parser::combinator::opt;
use crate::parser::{ParseResult, TokenList};
use crate::token::Token;

pub fn many<'s, T>(
    mut tokens: TokenList<'s>,
    mut parse_item: impl FnMut(TokenList<'s>) -> ParseResult<'s, T>,
) -> ParseResult<'s, Vec<T>> {
    let mut values = Vec::new();
    loop {
        match parse_item(tokens) {
            Ok((new_tokens, item)) => {
                tokens = new_tokens;
                values.push(item);
            }
            Err(err) if err.is_fatal => return Err(err),
            Err(_) => return Ok((tokens, values)),
        }
    }
}

pub fn separated_list1<'s, T>(
    tokens: TokenList<'s>,
    mut parse_item: impl FnMut(TokenList<'s>) -> ParseResult<'s, T>,
    mut parse_separator: impl FnMut(TokenList<'s>) -> ParseResult<'s, &'s Token<'s>>,
) -> ParseResult<'s, SeparatedList1<'s, T>> {
    let (mut tokens, first_item) = parse_item(tokens)?;
    let mut list = SeparatedList1 {
        items: Vec::new(),
        last_item: Box::new(first_item),
    };

    loop {
        let (next_tokens, separator) = match parse_separator(tokens) {
            Ok(vals) => vals,
            Err(err) if err.is_fatal => return Err(err),
            Err(_) => return Ok((tokens, list)),
        };

        let (next_tokens, next_item) = parse_item(next_tokens)?;
        tokens = next_tokens;
        list.push(separator, next_item);
    }
}

pub fn separated_list_trailing1<'s, T>(
    tokens: TokenList<'s>,
    mut parse_item: impl FnMut(TokenList<'s>) -> ParseResult<'s, T>,
    mut parse_separator: impl FnMut(TokenList<'s>) -> ParseResult<'s, &'s Token<'s>>,
) -> ParseResult<'s, SeparatedListTrailing1<'s, T>> {
    let (mut tokens, first_item) = parse_item(tokens)?;
    let mut list = SeparatedList1 {
        items: Vec::new(),
        last_item: Box::new(first_item),
    };

    loop {
        let (next_tokens, separator) = match parse_separator(tokens) {
            Ok(vals) => vals,
            Err(err) if err.is_fatal => return Err(err),
            Err(_) => return Ok((tokens, list.into_trailing(None))),
        };

        let (next_tokens, next_item) = match parse_item(next_tokens) {
            Ok(vals) => vals,
            Err(err) if err.is_fatal => return Err(err),
            Err(_) => return Ok((next_tokens, list.into_trailing(Some(separator)))),
        };

        tokens = next_tokens;
        list.push(separator, next_item);
    }
}

pub fn separated_list_trailing0<'s, T>(
    tokens: TokenList<'s>,
    parse_item: impl FnMut(TokenList<'s>) -> ParseResult<'s, T>,
    parse_separator: impl FnMut(TokenList<'s>) -> ParseResult<'s, &'s Token<'s>>,
) -> ParseResult<'s, SeparatedListTrailing0<'s, T>> {
    opt(
        tokens,
        separated_list_trailing1(tokens, parse_item, parse_separator),
    )
}
