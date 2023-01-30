use crate::ast::{ArrayValue, Precedence};
use crate::parser::combinator::opt;
use crate::parser::expression::expression;
use crate::parser::token::terminal;
use crate::parser::{ParseResult, TokenList};
use crate::token::TerminalToken;

pub fn array_value(tokens: TokenList) -> ParseResult<ArrayValue> {
    let (tokens, value) = expression(tokens, Precedence::Comma)?;
    let (tokens, separator) = opt(tokens, terminal(tokens, TerminalToken::Comma))?;
    Ok((
        tokens,
        ArrayValue {
            value: Box::new(value),
            separator,
        },
    ))
}
