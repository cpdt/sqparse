use crate::lexer::TokenItem;
use crate::parser::{ParseError, ParseErrorType, ParseResult, TokenList};
use crate::token::{TerminalToken, Token, TokenType};

fn is_terminal_of_type(token: &Token, expected: TerminalToken) -> bool {
    match token.ty {
        TokenType::Terminal(received) => received == expected,
        _ => false,
    }
}

pub fn terminal_item(tokens: TokenList, terminal: TerminalToken) -> ParseResult<&TokenItem> {
    if let Some((tokens, item)) = tokens.split_first() {
        if is_terminal_of_type(&item.token, terminal) {
            return Ok((tokens, item));
        }
    }

    Err(ParseError::new(
        ParseErrorType::ExpectedTerminal(terminal),
        tokens.start_index(),
    ))
}

pub fn terminal(tokens: TokenList, terminal: TerminalToken) -> ParseResult<&Token> {
    let (tokens, item) = terminal_item(tokens, terminal)?;
    Ok((tokens, &item.token))
}

pub fn terminal2(
    tokens: TokenList,
    terminal_a: TerminalToken,
    terminal_b: TerminalToken,
) -> ParseResult<(&Token, &Token)> {
    let index = tokens.start_index();
    let (tokens, token_a) = terminal(tokens, terminal_a)?;
    let (tokens, token_b) = terminal(tokens, terminal_b)?;

    if token_a.range.end == token_b.range.start {
        return Ok((tokens, (token_a, token_b)));
    }

    Err(ParseError::new(
        ParseErrorType::ExpectedCompound2(terminal_a, terminal_b),
        index,
    ))
}

pub fn terminal3(
    tokens: TokenList,
    terminal_a: TerminalToken,
    terminal_b: TerminalToken,
    terminal_c: TerminalToken,
) -> ParseResult<(&Token, &Token, &Token)> {
    let index = tokens.start_index();
    let (tokens, token_a) = terminal(tokens, terminal_a)?;
    let (tokens, token_b) = terminal(tokens, terminal_b)?;
    let (tokens, token_c) = terminal(tokens, terminal_c)?;

    if token_a.range.end == token_b.range.start && token_b.range.end == token_c.range.start {
        return Ok((tokens, (token_a, token_b, token_c)));
    }

    Err(ParseError::new(
        ParseErrorType::ExpectedCompound3(terminal_a, terminal_b, terminal_c),
        index,
    ))
}
