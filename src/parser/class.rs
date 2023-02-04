use crate::ast::{ClassDefinition, ClassExtends, ClassMember, Precedence};
use crate::parser::expression::{expression, table_delimited};
use crate::parser::parse_result_ext::ParseResultExt;
use crate::parser::slot::slot;
use crate::parser::token_list::TokenList;
use crate::parser::token_list_ext::TokenListExt;
use crate::parser::ParseResult;
use crate::token::TerminalToken;
use crate::{ContextType, ParseErrorType};

pub fn class_definition(tokens: TokenList) -> ParseResult<ClassDefinition> {
    let (tokens, extends) = class_extends(tokens).maybe(tokens)?;
    let (tokens, (open, members, close)) = tokens.terminal(TerminalToken::OpenBrace).opens(
        ContextType::Span,
        |tokens| tokens.terminal(TerminalToken::CloseBrace),
        |tokens, open, close| {
            tokens
                .many(class_member)
                .map_val(|members| (open, members, close))
        },
    )?;

    Ok((
        tokens,
        ClassDefinition {
            extends,
            open,
            members,
            close,
        },
    ))
}

pub fn class_extends(tokens: TokenList) -> ParseResult<ClassExtends> {
    let (tokens, extends) = tokens.terminal(TerminalToken::Extends)?;
    let (tokens, name) = expression(tokens, Precedence::None)?;
    Ok((tokens, ClassExtends { extends, name }))
}

pub fn class_member(tokens: TokenList) -> ParseResult<ClassMember> {
    let (tokens, attributes) = table_delimited(
        tokens,
        TerminalToken::OpenAttributes,
        TerminalToken::CloseAttributes,
    )
    .maybe(tokens)?;
    let (tokens, static_) = tokens.terminal(TerminalToken::Static).maybe(tokens)?;
    let (tokens, slot) =
        slot(tokens).or_error(|| tokens.error(ParseErrorType::ExpectedClassMember))?;
    let (tokens, semicolon) = tokens.terminal(TerminalToken::Semicolon).maybe(tokens)?;
    Ok((
        tokens,
        ClassMember {
            attributes,
            static_,
            slot,
            semicolon,
        },
    ))
}
