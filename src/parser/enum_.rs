use crate::ast::EnumEntry;
use crate::parser::combinator::opt;
use crate::parser::identifier::identifier;
use crate::parser::token::terminal;
use crate::parser::variable::var_initializer;
use crate::parser::{ParseResult, TokenList};
use crate::token::TerminalToken;

pub fn enum_entry(tokens: TokenList) -> ParseResult<EnumEntry> {
    let (tokens, name) = identifier(tokens)?;
    let (tokens, initializer) = opt(tokens, var_initializer(tokens))?;
    let (tokens, comma) = opt(tokens, terminal(tokens, TerminalToken::Comma))?;
    Ok((
        tokens,
        EnumEntry {
            name,
            initializer,
            comma,
        },
    ))
}
