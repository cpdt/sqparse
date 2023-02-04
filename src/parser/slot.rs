use crate::ast::{Precedence, Slot};
use crate::parser::expression::expression;
use crate::parser::function::function_definition;
use crate::parser::identifier::identifier;
use crate::parser::parse_result_ext::ParseResultExt;
use crate::parser::token_list::TokenList;
use crate::parser::token_list_ext::TokenListExt;
use crate::parser::type_::type_;
use crate::parser::variable::var_initializer;
use crate::parser::ParseResult;
use crate::token::TerminalToken;
use crate::{ContextType, ParseErrorType};

pub fn slot(tokens: TokenList) -> ParseResult<Slot> {
    // `constructor_slot` must go before `function_slot` to ensure "function constructor(..)" is
    // parsed correctly.
    // `function_slot` must go before anything that can look like a type, to ensure the return type
    // is parsed.
    constructor_slot(tokens)
        .or_try(|| function_slot(tokens))
        .or_try(|| property_slot(tokens))
        .or_try(|| computed_property_slot(tokens))
        .or_error(|| tokens.error(ParseErrorType::ExpectedSlot))
}

fn property_slot(tokens: TokenList) -> ParseResult<Slot> {
    identifier(tokens)
        .determines(|tokens, name| {
            var_initializer(tokens).map_val(|initializer| Slot::Property { name, initializer })
        })
        .with_context_from(ContextType::Property, tokens)
}

fn computed_property_slot(tokens: TokenList) -> ParseResult<Slot> {
    tokens
        .terminal(TerminalToken::OpenSquare)
        .opens(
            ContextType::Span,
            |tokens| tokens.terminal(TerminalToken::CloseSquare),
            |tokens, open, close| {
                expression(tokens, Precedence::None).map_val(|name| (open, name, close))
            },
        )
        .determines(|tokens, (open, name, close)| {
            var_initializer(tokens).map_val(|initializer| Slot::ComputedProperty {
                open,
                name,
                close,
                initializer,
            })
        })
        .with_context_from(ContextType::Property, tokens)
}

fn constructor_slot(tokens: TokenList) -> ParseResult<Slot> {
    tokens
        .terminal(TerminalToken::Function)
        .maybe(tokens)
        .and_then(|(tokens, function)| {
            tokens
                .terminal(TerminalToken::Constructor)
                .map_val(|constructor| (function, constructor))
        })
        .determines(|tokens, (function, constructor)| {
            function_definition(tokens).map_val(|definition| Slot::Constructor {
                function,
                constructor,
                definition: Box::new(definition),
            })
        })
        .with_context_from(ContextType::Constructor, tokens)
}

fn function_slot(tokens: TokenList) -> ParseResult<Slot> {
    type_(tokens)
        .not_line_ending()
        .not_definite()
        .maybe(tokens)
        .and_then(|(tokens, return_type)| {
            tokens
                .terminal(TerminalToken::Function)
                .map_val(|function| (return_type, function))
        })
        .determines(|tokens, (return_type, function)| {
            let (tokens, name) = identifier(tokens)?;
            let (tokens, definition) = function_definition(tokens)?;
            Ok((
                tokens,
                Slot::Function {
                    return_type,
                    function,
                    name,
                    definition: Box::new(definition),
                },
            ))
        })
}
