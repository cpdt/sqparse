use crate::ast::{Precedence, VarDefinition, VarInitializer};
use crate::parser::expression::expression;
use crate::parser::identifier::identifier;
use crate::parser::parse_result_ext::ParseResultExt;
use crate::parser::token_list::TokenList;
use crate::parser::token_list_ext::TokenListExt;
use crate::parser::ParseResult;
use crate::token::TerminalToken;

pub fn var_definition(tokens: TokenList) -> ParseResult<VarDefinition> {
    let (tokens, name) = identifier(tokens)?;
    let (tokens, initializer) = var_initializer(tokens).maybe(tokens)?;
    Ok((tokens, VarDefinition { name, initializer }))
}

pub fn var_initializer(tokens: TokenList) -> ParseResult<VarInitializer> {
    tokens
        .terminal(TerminalToken::Assign)
        .determines(|tokens, assign| {
            expression(tokens, Precedence::Comma).map_val(|value| VarInitializer { assign, value })
        })
}
