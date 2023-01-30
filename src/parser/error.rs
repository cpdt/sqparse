use crate::parser::context::ContextType;
use crate::token::TerminalToken;
use crate::{display_error, TokenItem};
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParseErrorType {
    Internal(InternalErrorType),
    ExpectedTerminal(TerminalToken),
    ExpectedCompound2(TerminalToken, TerminalToken),
    ExpectedCompound3(TerminalToken, TerminalToken, TerminalToken),
    ExpectedIdentifier,
    ExpectedLiteral,

    ExpectedExpression,
    ExpectedOperator,
    ExpectedPrefixOperator,
    ExpectedPostfixOperator,
    ExpectedBinaryOperator,

    ExpectedType,
    ExpectedTypeModifier,

    ExpectedTableSlot,
    ExpectedClassMember,

    ExpectedStatement,
    ExpectedGlobalDeclaration,

    IllegalLineBreak,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InternalErrorType {
    TokenIsNotSpan,
    SpanEndPastEof,
    PrecedenceMismatch,
    Empty,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub ty: ParseErrorType,
    pub token_index: usize,
    pub context: Option<ParseErrorContext>,
    pub is_fatal: bool,
}

#[derive(Debug, Clone)]
pub struct ParseErrorContext {
    pub token_range: Range<usize>,
    pub ty: ContextType,
}

impl ParseError {
    pub fn new(ty: ParseErrorType, token_index: usize) -> Self {
        ParseError {
            ty,
            token_index,
            context: None,
            is_fatal: false,
        }
    }

    pub fn with_context(mut self, token_range: Range<usize>, ty: ContextType) -> Self {
        let replace = match &self.context {
            Some(context) => !context.ty.is_useful(),
            None => true,
        };
        if replace {
            match &mut self.context {
                Some(context) => {
                    context.token_range.start = context.token_range.start.min(token_range.start);
                    context.token_range.end = context.token_range.end.max(token_range.end);
                    context.ty = ty;
                }
                None => self.context = Some(ParseErrorContext { token_range, ty }),
            }
        }
        self
    }

    pub fn into_fatal(mut self) -> Self {
        self.is_fatal = true;
        self
    }

    pub fn into_non_fatal(mut self) -> Self {
        self.is_fatal = false;
        self
    }

    pub fn display<'s>(
        &'s self,
        source: &'s str,
        tokens: &'s [TokenItem<'s>],
    ) -> impl std::fmt::Display + 's {
        Display {
            error: self,
            source,
            tokens,
        }
    }
}

impl std::fmt::Display for ParseErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorType::Internal(val) => write!(
                f,
                "internal error {val:?} - this should not be shown. Please report a bug at https://github.com/cpdt/sqfmt/issues with sample code.",
            ),
            ParseErrorType::ExpectedTerminal(terminal) => write!(f, "expected `{}`", terminal.as_str()),
            ParseErrorType::ExpectedCompound2(token1, token2) => write!(f, "expected `{}{}`", token1.as_str(), token2.as_str()),
            ParseErrorType::ExpectedCompound3(token1, token2, token3) => write!(f, "expected `{}{}{}`", token1.as_str(), token2.as_str(), token3.as_str()),
            ParseErrorType::ExpectedIdentifier => write!(f, "expected an identifier"),
            ParseErrorType::ExpectedLiteral => write!(f, "expected a literal"),

            ParseErrorType::ExpectedExpression => write!(f, "expected an expression"),
            ParseErrorType::ExpectedOperator => write!(f, "expected an operator"),
            ParseErrorType::ExpectedPrefixOperator => write!(f, "expected a prefix operator"),
            ParseErrorType::ExpectedPostfixOperator => write!(f, "expected a postfix operator"),
            ParseErrorType::ExpectedBinaryOperator => write!(f, "expected a binary operator"),

            ParseErrorType::ExpectedType => write!(f, "expected a type"),
            ParseErrorType::ExpectedTypeModifier => write!(f, "expected a type modifier"),

            ParseErrorType::ExpectedTableSlot => write!(f, "expected a table slot"),
            ParseErrorType::ExpectedClassMember => write!(f, "expected a class member"),

            ParseErrorType::ExpectedStatement => write!(f, "expected a statement"),
            ParseErrorType::ExpectedGlobalDeclaration => write!(f, "expected `function`, `const`, `enum`, `class`, `struct`, `typedef`, or a type"),

            ParseErrorType::IllegalLineBreak => write!(f, "expected anything but `\n`; got it anyway")
        }
    }
}

struct Display<'s> {
    error: &'s ParseError,
    source: &'s str,
    tokens: &'s [TokenItem<'s>],
}

impl std::fmt::Display for Display<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let src_range = token_src_range(self.error.token_index, self.tokens, self.source);
        write!(
            f,
            "{} {}",
            display_error(src_range, self.source),
            self.error.ty
        )?;

        if let Some(context) = &self.error.context {
            let start_range = token_src_range(context.token_range.start, self.tokens, self.source);
            let end_range = token_src_range(context.token_range.end - 1, self.tokens, self.source);
            writeln!(f)?;
            writeln!(f)?;
            write!(
                f,
                "{} in this {}",
                display_error(start_range.start..end_range.end, self.source),
                context.ty,
            )?;
        }

        Ok(())
    }
}

fn token_src_range(token_index: usize, tokens: &[TokenItem], src: &str) -> Range<usize> {
    match tokens.get(token_index) {
        Some(item) => item.token.range.clone(),
        None => src.len()..src.len(),
    }
}
