use crate::ast::{StructDefinition, StructProperty};
use crate::parser::identifier::identifier;
use crate::parser::parse_result_ext::ParseResultExt;
use crate::parser::token_list::TokenList;
use crate::parser::token_list_ext::TokenListExt;
use crate::parser::type_::type_;
use crate::parser::variable::var_initializer;
use crate::parser::ParseResult;
use crate::token::TerminalToken;
use crate::ContextType;

pub fn struct_definition(tokens: TokenList) -> ParseResult<StructDefinition> {
    tokens.terminal(TerminalToken::OpenBrace).opens(
        ContextType::Span,
        |tokens| tokens.terminal(TerminalToken::CloseBrace),
        |tokens, open, close| {
            tokens
                .many(struct_property)
                .map_val(|properties| StructDefinition {
                    open,
                    properties,
                    close,
                })
        },
    )
}

pub fn struct_property(tokens: TokenList) -> ParseResult<StructProperty> {
    type_(tokens)
        .determines(|tokens, type_| {
            let (tokens, name) = identifier(tokens)?;
            let (tokens, initializer) = var_initializer(tokens).maybe(tokens)?;
            let (tokens, comma) = tokens.terminal(TerminalToken::Comma).maybe(tokens)?;
            Ok((
                tokens,
                StructProperty {
                    type_,
                    name,
                    initializer,
                    comma,
                },
            ))
        })
        .with_context_from(ContextType::Property, tokens)
}
