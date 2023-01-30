use crate::ast::{StructDeclaration, StructProperty};
use crate::parser::combinator::{opt, span};
use crate::parser::identifier::identifier;
use crate::parser::list::many;
use crate::parser::token::terminal;
use crate::parser::type_::type_;
use crate::parser::variable::var_initializer;
use crate::parser::{ContextType, ParseResult, TokenList};
use crate::token::TerminalToken;

pub fn struct_declaration(tokens: TokenList) -> ParseResult<StructDeclaration> {
    span(
        tokens,
        ContextType::StructDeclaration,
        TerminalToken::OpenBrace,
        TerminalToken::CloseBrace,
        |tokens, open, close| {
            let (tokens, properties) = many(tokens, struct_property)?;
            Ok((
                tokens,
                StructDeclaration {
                    open,
                    properties,
                    close,
                },
            ))
        },
    )
}

pub fn struct_property(tokens: TokenList) -> ParseResult<StructProperty> {
    let (tokens, ty) = type_(tokens)?;
    let (tokens, name) = identifier(tokens)?;
    let (tokens, initializer) = opt(tokens, var_initializer(tokens))?;
    let (tokens, comma) = opt(tokens, terminal(tokens, TerminalToken::Comma))?;

    Ok((
        tokens,
        StructProperty {
            ty,
            name,
            initializer,
            comma,
        },
    ))
}
