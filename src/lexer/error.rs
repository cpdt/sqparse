use crate::display_error;
use crate::token::TokenType;
use std::ops::Range;

/// Type of [`LexerError`].
///
/// Implements [`std::fmt::Display`] to write a useful error message.
#[derive(Debug, Clone, Copy)]
pub enum LexerErrorType<'s> {
    /// The input string ended in the middle of a string.
    ///
    /// # Example
    /// ```text
    /// "this string never ends
    ///                         ^ error
    /// ```
    EndOfInputInsideString,

    /// The line ended in the middle of a non-verbatim string.
    ///
    /// # Example
    /// ```text
    /// "this text spans multiple
    /// lines"                    ^ error
    /// ```
    EndOfLineInsideString,

    /// The input string ended in the middle of a multi-line comment.
    ///
    /// # Example
    /// ```text
    /// /* this comment never ends
    ///                            ^ error
    /// ```
    EndOfInputInsideComment,

    /// Something in the input was not recognized as a valid token.
    ///
    /// # Example
    /// ```text
    /// what even is this -> @
    ///                      ^ error
    /// ```
    InvalidInput,

    /// An opener token was not matched by a closing token.
    ///
    /// # Example
    /// ```text
    /// myfunc(1, 2
    ///       ^ error
    /// ```
    UnmatchedOpener {
        open: TokenType<'s>,
        close: TokenType<'s>,
    },
}

/// An error emitted while trying to tokenize an input string.
///
/// Each error has a type with more information, and a range indicating where in the source string
/// the error occurred.
#[derive(Debug, Clone)]
pub struct LexerError<'s> {
    /// The type of error.
    pub ty: LexerErrorType<'s>,

    /// The character range of where the error occurred.
    pub range: Range<usize>,
}

impl<'s> LexerError<'s> {
    /// Creates a new `LexerError`.
    pub fn new(ty: LexerErrorType<'s>, range: Range<usize>) -> Self {
        LexerError { ty, range }
    }

    /// Returns an implementation of [`std::fmt::Display`] that pretty-prints the error with source
    /// context using [`display_error`].
    pub fn display<'a>(&'a self, source: &'a str) -> impl std::fmt::Display + 'a {
        Display {
            error: self,
            source,
        }
    }
}

impl std::fmt::Display for LexerErrorType<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerErrorType::EndOfInputInsideString => {
                write!(f, "input ended in the middle of a string")
            }
            LexerErrorType::EndOfLineInsideString => {
                write!(f, "strings cannot span multiple lines")
            }
            LexerErrorType::EndOfInputInsideComment => {
                write!(f, "input ended in the middle of a comment")
            }
            LexerErrorType::InvalidInput => write!(f, "not sure what this is"),
            LexerErrorType::UnmatchedOpener { close, .. } => {
                write!(f, "missing a closing {close}")
            }
        }
    }
}

struct Display<'s> {
    error: &'s LexerError<'s>,
    source: &'s str,
}

impl std::fmt::Display for Display<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            display_error(self.error.range.clone(), self.source),
            self.error.ty
        )
    }
}
