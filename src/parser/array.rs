use crate::ast::{ArrayValue, Precedence};
use crate::parser::expression::expression;
use crate::parser::parse_result_ext::ParseResultExt;
use crate::parser::token_list_ext::TokenListExt;
use crate::parser::{ParseResult, TokenList};
use crate::token::TerminalToken;

pub fn array_value(tokens: TokenList) -> ParseResult<ArrayValue> {
    let (tokens, value) = expression(tokens, Precedence::Comma)?;
    let (tokens, separator) = tokens.terminal(TerminalToken::Comma).maybe(tokens)?;
    Ok((tokens, ArrayValue { value, separator }))
}
