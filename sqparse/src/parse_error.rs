use crate::error_display::error;
use crate::token::TerminalToken;
use crate::{Token, TokenList};
use nom::error::ErrorKind;
use std::fmt::Formatter;

#[derive(Debug, Clone, Copy)]
pub enum ErrorType {
    Internal(ErrorKind),
    ExpectedTerminal(TerminalToken),
    ExpectedCompound2(TerminalToken, TerminalToken),
    ExpectedCompound3(TerminalToken, TerminalToken, TerminalToken),
    ExpectedIdentifier,
    ExpectedLiteral,
}

#[derive(Debug, Clone, Copy)]
struct ErrorContext {
    token_offset: usize,
    context: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct ParseError {
    token_offset: usize,
    ty: ErrorType,
    context: Option<ErrorContext>,
}

impl ParseError {
    pub fn new(token_offset: usize, ty: ErrorType) -> Self {
        ParseError {
            token_offset,
            ty,
            context: None,
        }
    }

    pub fn display<'s>(&'s self, tokens: &'s [Token<'s>], source: &'s str) -> Display<'s> {
        Display {
            error: self,
            tokens,
            source,
        }
    }
}

impl<'s> nom::error::ParseError<TokenList<'s>> for ParseError {
    fn from_error_kind(input: TokenList<'s>, kind: ErrorKind) -> Self {
        ParseError {
            token_offset: input.get_offset(),
            ty: ErrorType::Internal(kind),
            context: None,
        }
    }

    fn append(_input: TokenList<'s>, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'s> nom::error::ContextError<TokenList<'s>> for ParseError {
    fn add_context(input: TokenList<'s>, context: &'static str, mut other: Self) -> Self {
        if other.context.is_none() {
            other.context = Some(ErrorContext {
                token_offset: input.get_offset() - 1,
                context,
            });
        }
        other
    }
}

#[derive(Clone, Copy)]
pub struct Display<'s> {
    error: &'s ParseError,
    tokens: &'s [Token<'s>],
    source: &'s str,
}

impl<'s> std::fmt::Display for Display<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "parse error:")?;
        writeln!(
            f,
            "{token} {description}",
            token = TokenDisplay {
                token_offset: self.error.token_offset,
                tokens: self.tokens,
                source: self.source
            },
            description = self.error.ty,
        )?;

        if let Some(context) = self.error.context {
            writeln!(f)?;
            writeln!(
                f,
                "{token} in this {context}",
                token = TokenDisplay {
                    token_offset: context.token_offset,
                    tokens: self.tokens,
                    source: self.source
                },
                context = context.context,
            )?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
struct TokenDisplay<'s> {
    token_offset: usize,
    tokens: &'s [Token<'s>],
    source: &'s str,
}

impl<'s> std::fmt::Display for TokenDisplay<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let token_range = match self.tokens.get(self.token_offset) {
            Some(token) => token.range.clone(),
            None => (self.source.len()..(self.source.len() + 1)),
        };

        write!(f, "{}", error(token_range, self.source))
    }
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::Internal(val) => write!(
                f,
                "{} error - this is an internal error and should not be shown. Please report a bug at https://github.com/cpdt/sqfmt/issues with sample code.",
                val.description(),
            ),
            ErrorType::ExpectedTerminal(token) => write!(f, "expected `{}`", token.as_str()),
            ErrorType::ExpectedCompound2(token1, token2) => write!(f, "expected `{}{}`", token1.as_str(), token2.as_str()),
            ErrorType::ExpectedCompound3(token1, token2, token3) => write!(f, "expected `{}{}{}`", token1.as_str(), token2.as_str(), token3.as_str()),
            ErrorType::ExpectedIdentifier => write!(f, "expected an identifier"),
            ErrorType::ExpectedLiteral => write!(f, "expected a literal"),
        }
    }
}
