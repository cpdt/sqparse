use crate::lexer::parse_str::ParseStr;
use crate::token::TerminalToken;
use crate::Flavor;

pub fn try_symbol(val: ParseStr, flavor: Flavor) -> Option<(TerminalToken, ParseStr)> {
    TerminalToken::SYMBOLS
        .iter()
        .filter(|(token, _)| token.is_supported(flavor))
        .find(|(_, token_val)| val.as_str().starts_with(token_val))
        .map(|(token, token_val)| (*token, val.from(token_val.len())))
}
