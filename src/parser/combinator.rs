use crate::parser::error::InternalErrorType;
use crate::parser::token::{terminal, terminal_item};
use crate::parser::{ContextType, ParseError, ParseErrorType, ParseResult, TokenList};
use crate::token::{TerminalToken, Token};

pub fn definitely<'s, D, T>(
    tokens: TokenList<'s>,
    context: ContextType,
    determinant: impl FnOnce(TokenList<'s>) -> ParseResult<'s, D>,
    remaining: impl FnOnce(TokenList<'s>, D) -> ParseResult<'s, T>,
) -> ParseResult<'s, T> {
    let start_index = tokens.start_index();
    let (rem_tokens, d) = determinant(tokens)?;
    remaining(rem_tokens, d).map_err(|err| {
        let index = err.token_index + 1;
        err.with_context(start_index..index, context).into_fatal()
    })
}

pub fn first_of<'s, T, P: FnOnce(TokenList<'s>) -> ParseResult<'s, T>>(
    tokens: TokenList<'s>,
    parsers: impl IntoIterator<Item = P>,
    default_parser: impl FnOnce(TokenList<'s>) -> ParseResult<'s, T>,
) -> ParseResult<'s, T> {
    for parser in parsers {
        match parser(tokens) {
            Ok((tokens, value)) => return Ok((tokens, value)),
            Err(err) if err.is_fatal => return Err(err),
            Err(_) => {}
        }
    }
    default_parser(tokens)
}

pub fn span<'s, T>(
    tokens: TokenList<'s>,
    context: ContextType,
    open_terminal: TerminalToken,
    close_terminal: TerminalToken,
    inner: impl FnOnce(TokenList<'s>, &'s Token<'s>, &'s Token<'s>) -> ParseResult<'s, T>,
) -> ParseResult<'s, T> {
    let start_index = tokens.start_index();
    let (next_tokens, open_item) = terminal_item(tokens, open_terminal)?;
    let Some(close_index) = open_item.close_index else {
        return Err(ParseError::new(ParseErrorType::Internal(InternalErrorType::TokenIsNotSpan), start_index));
    };

    let (inner_tokens, outer_tokens) = next_tokens.split_at(close_index);
    let close_index = outer_tokens.start_index() + 1;
    let (outer_tokens, close_token) = terminal(outer_tokens, close_terminal).map_err(|err| {
        err.with_context(start_index..close_index, context)
            .into_fatal()
    })?;

    let (inner_tokens, value) =
        inner(inner_tokens, &open_item.token, close_token).map_err(|err| {
            err.with_context(start_index..close_index, context)
                .into_fatal()
        })?;
    if !inner_tokens.is_ended() {
        return Err(ParseError::new(
            ParseErrorType::ExpectedTerminal(close_terminal),
            inner_tokens.start_index(),
        )
        .with_context(start_index..close_index, context)
        .into_fatal());
    }

    Ok((outer_tokens, value))
}

pub fn opt<'s, T>(tokens: TokenList<'s>, res: ParseResult<'s, T>) -> ParseResult<'s, Option<T>> {
    match res {
        Ok((tokens, val)) => Ok((tokens, Some(val))),
        Err(err) if err.is_fatal => Err(err),
        Err(_) => Ok((tokens, None)),
    }
}

pub fn map<A, B>(res: ParseResult<A>, map: impl FnOnce(A) -> B) -> ParseResult<B> {
    res.map(|(tokens, a)| (tokens, map(a)))
}

pub fn alt<T>(res: ParseResult<T>) -> Option<ParseResult<T>> {
    match res {
        Ok((tokens, a)) => Some(Ok((tokens, a))),
        Err(err) if err.is_fatal => Some(Err(err)),
        Err(_) => None,
    }
}

pub fn alt_map<A, B>(res: ParseResult<A>, mapper: impl FnOnce(A) -> B) -> Option<ParseResult<B>> {
    alt(map(res, mapper))
}

pub fn prevent_ending_line<'s, T>(
    tokens: TokenList<'s>,
    res: ParseResult<'s, T>,
) -> ParseResult<'s, T> {
    let (new_tokens, val) = res?;
    if tokens.start_index() != new_tokens.start_index() && new_tokens.is_newline() {
        Err(ParseError::new(
            ParseErrorType::IllegalLineBreak,
            new_tokens.start_index() - 1,
        ))
    } else {
        Ok((new_tokens, val))
    }
}
