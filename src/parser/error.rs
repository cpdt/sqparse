use crate::parser::context::ContextType;
use crate::token::TerminalToken;
use crate::{display_error, TokenItem};
use std::ops::Range;

/// Type of [`ParseError`].
///
/// Implements [`std::fmt::Display`] to write a useful error message.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParseErrorType {
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

    /// Expected a token that starts a value in an expression but got something else.
    ///
    /// # Example
    /// ```text
    /// local sum = 1 + ?
    ///                 ^ error
    /// ```
    ExpectedValue,

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

    /// Expected a newline or semicolon to end a statement but got something else.
    ///
    /// # Example
    /// ```text
    /// { 1 } + 2
    ///       ^ error
    /// ```
    ExpectedEndOfStatement,

    /// Expected a token that starts a global definition but got something else.
    ///
    /// # Example
    /// ```text
    /// global if ()
    ///        ^ error
    /// ```
    ExpectedGlobalDefinition,

    /// Found a linebreak in a place where one is not allowed.
    IllegalLineBreak,

    /// An expression was not allowed due to precedence rules.
    Precedence,

    /// Expected a slot in a class or table.
    ExpectedSlot,

    /// Expected a string literal.
    ExpectedStringLiteral,
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

    /// Affinity of the token.
    pub token_affinity: TokenAffinity,

    /// Contextual information if available.
    pub context: Option<ParseErrorContext>,

    pub(crate) is_fatal: bool,
}

/// Affinity of the token index in [`ParseError`].
///
/// This controls how a token range is printed when the token is at the start of a newline. For
/// example, in this input:
/// ```text
/// a +
/// b
/// ```
///
/// An affinity of [`Before`] on `b` would highlight the end of the line before `b`, indicating the
/// error is not necessarily related to `b` itself but to something missing after `+`:
/// ```text
/// a +
///    ^
/// b
/// ```
///
/// An affinity of [`Inline`] would highlight `b` itself, indicating it is the problematic token:
/// ```text
/// a +
/// b
/// ^
/// ```
///
/// [`Before`]: TokenAffinity::Before
/// [`Inline`]: TokenAffinity::Inline
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenAffinity {
    Before,
    Inline,
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
/// So it will attach a context to the error with an [`Expression`] context.
///
/// [`Expression`]: ContextType::Expression
#[derive(Debug, Clone)]
pub struct ParseErrorContext {
    /// The range of tokens that this context applies.
    ///
    /// For example, if the context is a [`FunctionDefinitionStatement`], the range will include
    /// the entire function.
    ///
    /// In some cases this will end at the token where the error is encountered, however in many
    /// cases the parser can match delimiters like `{` and `}` to provide more context.
    ///
    /// [`FunctionDefinitionStatement`]: crate::ast::FunctionDefinitionStatement
    pub token_range: Range<usize>,

    /// Affinity of the last token in the range.
    pub end_affinity: TokenAffinity,

    /// The type of context.
    pub ty: ContextType,
}

impl ParseError {
    /// Creates a new `ParseError`.
    pub fn new(ty: ParseErrorType, token_index: usize, token_affinity: TokenAffinity) -> Self {
        ParseError {
            ty,
            token_index,
            token_affinity,
            context: None,
            is_fatal: false,
        }
    }

    /// Attaches some context to the error.
    pub fn with_context(
        self,
        ty: ContextType,
        token_range: Range<usize>,
        end_affinity: TokenAffinity,
    ) -> Self {
        self.replace_context(ContextType::Span, ty, token_range, end_affinity)
    }

    /// Replaces an existing context with a new one, if it matches.
    pub fn replace_context(
        mut self,
        from_ty: ContextType,
        to_ty: ContextType,
        token_range: Range<usize>,
        end_affinity: TokenAffinity,
    ) -> Self {
        // Sanity check, ensure the range includes the actual token.
        let token_range = (self.token_index.min(token_range.start))
            ..((self.token_index + 1).max(token_range.end));

        match &mut self.context {
            // Set a new context if there isn't one already.
            None => {
                self.context = Some(ParseErrorContext {
                    token_range,
                    ty: to_ty,
                    end_affinity,
                });
            }

            // Replace the existing context if it matches the replace type.
            Some(context) if context.ty == from_ty => {
                // Ensure the range contains both, allowing an inner context to expand the outer context.
                let token_range = (token_range.start.min(context.token_range.start))
                    ..(token_range.end.max(context.token_range.end));
                let end_affinity = if context.end_affinity == TokenAffinity::Inline {
                    TokenAffinity::Inline
                } else {
                    end_affinity
                };
                *context = ParseErrorContext {
                    token_range,
                    ty: to_ty,
                    end_affinity,
                };
            }

            // Otherwise, leave the existing context intact.
            _ => {}
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
            ParseErrorType::ExpectedTerminal(terminal) => {
                write!(f, "expected `{}`", terminal.as_str())
            }
            ParseErrorType::ExpectedCompound2(token1, token2) => {
                write!(f, "expected `{}{}`", token1.as_str(), token2.as_str())
            }
            ParseErrorType::ExpectedCompound3(token1, token2, token3) => write!(
                f,
                "expected `{}{}{}`",
                token1.as_str(),
                token2.as_str(),
                token3.as_str()
            ),
            ParseErrorType::ExpectedIdentifier => write!(f, "expected an identifier"),
            ParseErrorType::ExpectedLiteral => write!(f, "expected a literal"),
            ParseErrorType::ExpectedExpression => write!(f, "expected an expression"),
            ParseErrorType::ExpectedValue => write!(f, "expected a value"),
            ParseErrorType::ExpectedOperator => write!(f, "expected an operator"),
            ParseErrorType::ExpectedPrefixOperator => write!(f, "expected a prefix operator"),
            ParseErrorType::ExpectedPostfixOperator => write!(f, "expected a postfix operator"),
            ParseErrorType::ExpectedBinaryOperator => write!(f, "expected a binary operator"),
            ParseErrorType::ExpectedType => write!(f, "expected a type"),
            ParseErrorType::ExpectedTypeModifier => write!(f, "expected a type modifier"),
            ParseErrorType::ExpectedTableSlot => write!(f, "expected a table slot"),
            ParseErrorType::ExpectedClassMember => write!(f, "expected a class member"),
            ParseErrorType::ExpectedStatement => write!(f, "expected a statement"),
            ParseErrorType::ExpectedEndOfStatement => {
                write!(f, "expected a newline or `;`")
            }
            ParseErrorType::ExpectedGlobalDefinition => write!(f, "expected a global definition"),
            ParseErrorType::IllegalLineBreak => {
                write!(f, "expected anything but `\\n`; got it anyway")
            }
            ParseErrorType::Precedence => write!(f, "not allowed due to precedence rules"),
            ParseErrorType::ExpectedSlot => write!(f, "expected a slot"),
            ParseErrorType::ExpectedStringLiteral => write!(f, "expected a string literal"),
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
        let src_range = token_src_range(
            self.error.token_index,
            self.error.token_affinity,
            self.tokens,
        );
        write!(
            f,
            "{} {}",
            display_error(src_range, self.source),
            self.error.ty
        )?;

        if let Some(context) = &self.error.context {
            let start_range = token_src_range(
                context.token_range.start,
                TokenAffinity::Inline,
                self.tokens,
            );
            let end_range = token_src_range(
                context.token_range.end - 1,
                context.end_affinity,
                self.tokens,
            );
            writeln!(f)?;
            writeln!(f)?;

            let err_display = display_error(start_range.start..end_range.end, self.source);

            if self.error.token_index + 1 == context.token_range.end
                && context.end_affinity == TokenAffinity::Before
            {
                write!(f, "{err_display} for this {}", context.ty)?;
            } else {
                write!(f, "{err_display} in this {}", context.ty,)?;
            }
        }

        Ok(())
    }
}

fn token_src_range(
    token_index: usize,
    affinity: TokenAffinity,
    tokens: &[TokenItem],
) -> Range<usize> {
    let Some(last_item) = tokens.last() else { return 0..0; };

    if affinity == TokenAffinity::Before {
        if token_index > 0 {
            let last_item = &tokens[token_index.min(tokens.len()) - 1];
            last_item.token.range.end..last_item.token.range.end
        } else {
            0..0
        }
    } else if token_index < tokens.len() {
        let item = &tokens[token_index];
        item.token.range.clone()
    } else {
        last_item.token.range.end..last_item.token.range.end
    }
}
