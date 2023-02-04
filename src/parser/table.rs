use crate::ast::{Precedence, TableSlot, TableSlotType};
use crate::parser::expression::expression;
use crate::parser::parse_result_ext::ParseResultExt;
use crate::parser::slot::slot;
use crate::parser::token_list::TokenList;
use crate::parser::token_list_ext::TokenListExt;
use crate::parser::ParseResult;
use crate::token::{LiteralToken, StringToken, TerminalToken, Token, TokenType};
use crate::{ContextType, ParseErrorType};

pub fn table_slot(tokens: TokenList) -> ParseResult<TableSlot> {
    let (tokens, ty) = table_slot_type(tokens)?;
    let (tokens, comma) = tokens.terminal(TerminalToken::Comma).maybe(tokens)?;
    Ok((tokens, TableSlot { ty, comma }))
}

fn table_slot_type(tokens: TokenList) -> ParseResult<TableSlotType> {
    json_property_slot(tokens)
        .or_try(|| slot(tokens).map_val(TableSlotType::Slot))
        .or_error(|| tokens.error(ParseErrorType::ExpectedTableSlot))
}

fn json_property_slot(tokens: TokenList) -> ParseResult<TableSlotType> {
    string_literal(tokens)
        .determines(|tokens, (name, name_token)| {
            let (tokens, colon) = tokens.terminal(TerminalToken::Colon)?;
            let (tokens, value) = expression(tokens, Precedence::Comma)?;
            Ok((
                tokens,
                TableSlotType::JsonProperty {
                    name,
                    name_token,
                    colon,
                    value,
                },
            ))
        })
        .with_context_from(ContextType::Property, tokens)
}

fn string_literal(tokens: TokenList) -> ParseResult<(&str, &Token)> {
    if let Some((tokens, item)) = tokens.split_first() {
        if let TokenType::Literal(LiteralToken::String(StringToken::Literal(name))) = item.token.ty
        {
            return Ok((tokens, (name, &item.token)));
        }
    }

    Err(tokens.error(ParseErrorType::ExpectedStringLiteral))
}
