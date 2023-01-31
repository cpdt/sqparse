use crate::parser::context::ContextType;
use crate::token::TerminalToken;
use crate::{display_error, TokenItem};
use std::ops::Range;

/// Type of [`ParseError`].
///
/// Implements [`std::fmt::Display`] to write a useful error message.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParseErrorType {
    /// An internal unexpected error occurred. This is a bug.
    Internal(InternalErrorType),

    /// Expected a specific terminal token but got something else.
    ///
    /// # Example
    /// ```text
    /// function MyFunc { }
    ///                 ^ error
    /// ```
    ExpectedTerminal(TerminalToken),

    /// Expected a specific compound terminal but got something else.
    ExpectedCompound2(TerminalToken, TerminalToken),

    /// Expected a specific compound terminal but got something else.
    ExpectedCompound3(TerminalToken, TerminalToken, TerminalToken),

    /// Expected an identifier but got something else.
    ///
    /// # Example
    /// ```text
    /// global var !?!?! = "guh??"
    ///            ^ error
    /// ```
    ExpectedIdentifier,

    /// Expected a literal but got something else.
    ExpectedLiteral,

    /// Expected a token that starts an expression but got something else.
    ///
    /// # Example
    /// ```text
    /// int What = globalize_all_functions
    ///            ^ error
    /// ```
    ExpectedExpression,

    /// Expected an operator but got something else.
    ExpectedOperator,

    /// Expected a prefix operator but got something else.
    ExpectedPrefixOperator,

    /// Expected a postfix operator but got something else.
    ExpectedPostfixOperator,

    /// Expected a binary operator but got something else.
    ExpectedBinaryOperator,

    /// Expected a type but got something else.
    ///
    /// # Example
    /// ```text
    /// typedef Five 5
    ///              ^ error
    /// ```
    ExpectedType,

    /// Expected a type modifier but got something else.
    ///
    /// # Example
    /// ```text
    /// typedef help table&-
    ///                    ^ error
    /// ```
    ExpectedTypeModifier,

    /// Expected a token that starts a table slot but got something else.
    ///
    /// # Example
    /// ```text
    /// my_table = {
    ///     class MyTableClass {}
    ///     ^ error
    /// }
    /// ```
    ExpectedTableSlot,

    /// Expected a token that starts a class member but got something else.
    ///
    /// # Example
    /// ```text
    /// class MyClass {
    ///     globalize_all_functions
    ///     ^ error
    /// }
    /// ```
    ExpectedClassMember,

    /// Expected a token that starts a statement but got something else.
    ///
    /// # Example
    /// ```text
    /// > hey
    /// ^ error
    /// ```
    ExpectedStatement,

    /// Expected a token that starts a global declaration but got something else.
    ///
    /// # Example
    /// ```text
    /// global if ()
    ///        ^ error
    /// ```
    ExpectedGlobalDeclaration,

    /// Found a linebreak in a place where one is not allowed.
    IllegalLineBreak,
}

/// An internal unexpected error occurred. If you get one of these, it's a bug.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InternalErrorType {
    TokenIsNotSpan,
    SpanEndPastEof,
    PrecedenceMismatch,
    Empty,
}

/// An error emitted while trying to parse a token list.
///
/// Each error has a type with more information, the token where the error occurred, and possibly
/// some contextual information.
#[derive(Debug, Clone)]
pub struct ParseError {
    /// The type of error.
    pub ty: ParseErrorType,

    /// The index of the token where the error occurred.
    pub token_index: usize,

    /// Contextual information if available.
    pub context: Option<ParseErrorContext>,

    pub(crate) is_fatal: bool,
}

/// Context attached to a [`ParseError`].
///
/// This is generally attached to an error when the parser knows the context of what it is parsing
/// with confidence.
///
/// # Example
/// In this code, the parser knows that it is parsing the RHS of an expression when the error
/// occurs.
/// ```text
/// 1 + function
///     ^ error
/// ```
/// So it will attach a context to the error with a [`ExpressionRightHandSide`] context.
///
/// [`ExpressionRightHandSide`]: ContextType::ExpressionRightHandSide
#[derive(Debug, Clone)]
pub struct ParseErrorContext {
    /// The range of tokens that this context applies.
    ///
    /// For example, if the context is a [`FunctionDeclarationStatement`], the range will include
    /// the entire function.
    ///
    /// In some cases this will end at the token where the error is encountered, however in many
    /// cases the parser can match delimiters like `{` and `}` to provide more context.
    ///
    /// [`FunctionDeclarationStatement`]: crate::ast::FunctionDeclarationStatement
    pub token_range: Range<usize>,

    /// The type of context.
    pub ty: ContextType,
}

impl ParseError {
    /// Creates a new `ParseError`.
    pub fn new(ty: ParseErrorType, token_index: usize) -> Self {
        ParseError {
            ty,
            token_index,
            context: None,
            is_fatal: false,
        }
    }

    /// Attaches some context to the error.
    ///
    /// If the error already has context attached, it will only be replaced if the new context
    /// is deemed more useful, based on [`ContextType::is_useful`].
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

    /// Returns an implementation of [`std::fmt::Display`] that pretty-prints the error and context
    /// using [`display_error`].
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

    pub(crate) fn into_fatal(mut self) -> Self {
        self.is_fatal = true;
        self
    }

    pub(crate) fn into_non_fatal(mut self) -> Self {
        self.is_fatal = false;
        self
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
