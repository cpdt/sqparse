use crate::ast::{Precedence, TableSlot, TableSlotType};
use crate::parser::combinator::{alternative, first_of, opt, span};
use crate::parser::expression::{expression, literal};
use crate::parser::function::function_declaration;
use crate::parser::identifier::identifier;
use crate::parser::token::terminal;
use crate::parser::type_::type_;
use crate::parser::variable::var_initializer;
use crate::parser::{ContextType, ParseError, ParseErrorType, ParseResult, TokenList};
use crate::token::TerminalToken;

pub fn table_slot(tokens: TokenList) -> ParseResult<TableSlot> {
    let (tokens, ty) = table_slot_type(tokens)?;
    let (tokens, separator) = opt(tokens, terminal(tokens, TerminalToken::Comma))?;
    Ok((tokens, TableSlot { ty, separator }))
}

fn table_slot_type(tokens: TokenList) -> ParseResult<TableSlotType> {
    first_of(
        tokens,
        [
            // Must be before other types to ensure the return type is parsed.
            function_table_slot,
            property_table_slot,
            computed_property_table_slot,
            json_property_table_slot,
            constructor_table_slot,
        ],
        |_| {
            Err(ParseError::new(
                ParseErrorType::ExpectedTableSlot,
                tokens.start_index(),
            ))
        },
    )
}

fn property_table_slot(tokens: TokenList) -> ParseResult<TableSlotType> {
    alternative(
        tokens,
        ContextType::PropertyTableSlot,
        identifier,
        |tokens, name| {
            let (tokens, initializer) = var_initializer(tokens)?;
            Ok((tokens, TableSlotType::Property { name, initializer }))
        },
    )
}

fn computed_property_table_slot(tokens: TokenList) -> ParseResult<TableSlotType> {
    alternative(
        tokens,
        ContextType::ComputedPropertyTableSlot,
        |tokens| {
            span(
                tokens,
                ContextType::ComputedPropertyTableSlot,
                TerminalToken::OpenSquare,
                TerminalToken::CloseSquare,
                |tokens, open, close| {
                    let (tokens, name) = expression(tokens, Precedence::None)?;
                    Ok((tokens, (open, name, close)))
                },
            )
        },
        |tokens, (open, name, close)| {
            let (tokens, initializer) = var_initializer(tokens)?;
            Ok((
                tokens,
                TableSlotType::ComputedProperty {
                    open,
                    name: Box::new(name),
                    close,
                    initializer,
                },
            ))
        },
    )
}

fn json_property_table_slot(tokens: TokenList) -> ParseResult<TableSlotType> {
    alternative(
        tokens,
        ContextType::JsonPropertyTableSlot,
        literal,
        |tokens, name| {
            let (tokens, colon) = terminal(tokens, TerminalToken::Colon)?;
            let (tokens, value) = expression(tokens, Precedence::Comma)?;
            Ok((
                tokens,
                TableSlotType::JsonProperty {
                    name,
                    colon,
                    value: Box::new(value),
                },
            ))
        },
    )
}

fn constructor_table_slot(tokens: TokenList) -> ParseResult<TableSlotType> {
    alternative(
        tokens,
        ContextType::FunctionTableSlot,
        |tokens| terminal(tokens, TerminalToken::Constructor),
        |tokens, constructor| {
            let (tokens, declaration) = function_declaration(tokens)?;
            Ok((
                tokens,
                TableSlotType::Constructor {
                    constructor,
                    declaration: Box::new(declaration),
                }
            ))
        }
    )
}

fn function_table_slot(tokens: TokenList) -> ParseResult<TableSlotType> {
    alternative(
        tokens,
        ContextType::FunctionTableSlot,
        |tokens| {
            let (tokens, return_type) = opt(tokens, type_(tokens))?;
            let (tokens, function) = terminal(tokens, TerminalToken::Function)?;
            Ok((tokens, (return_type, function)))
        },
        |tokens, (return_type, function)| {
            let (tokens, name) = identifier(tokens)?;
            let (tokens, declaration) = function_declaration(tokens)?;
            Ok((
                tokens,
                TableSlotType::Function {
                    return_type,
                    function,
                    name,
                    declaration: Box::new(declaration),
                },
            ))
        },
    )
}
