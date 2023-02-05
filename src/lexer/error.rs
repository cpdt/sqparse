use crate::annotation::Mode;
use crate::token::TokenType;
use crate::{display_annotations, Annotation};
use owo_colors::OwoColorize;
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
    /// context using [`display_annotations`].
    pub fn display<'a>(
        &'a self,
        source: &'a str,
        file_name: Option<&'s str>,
    ) -> impl std::fmt::Display + 'a {
        Display {
            error: self,
            source,
            file_name,
        }
    }
}

impl<'a> LexerErrorType<'a> {
    pub fn inline_display(self) -> impl std::fmt::Display + 'a {
        LexerErrorInlineDisplay(self)
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
            LexerErrorType::InvalidInput => write!(f, "unrecognized token"),
            LexerErrorType::UnmatchedOpener { open, .. } => {
                write!(f, "unclosed delimiter {open}")
            }
        }
    }
}

struct LexerErrorInlineDisplay<'a>(LexerErrorType<'a>);

impl std::fmt::Display for LexerErrorInlineDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            LexerErrorType::EndOfInputInsideString | LexerErrorType::EndOfLineInsideString => {
                write!(f, "help: add a `\"`")
            }
            LexerErrorType::UnmatchedOpener { close, .. } => {
                write!(f, "help: add a closing {close}")
            }
            LexerErrorType::InvalidInput => write!(f, "not sure what this is"),
        }
    }
}

struct Display<'s> {
    error: &'s LexerError<'s>,
    source: &'s str,
    file_name: Option<&'s str>,
}

impl std::fmt::Display for Display<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{}",
            "error".bright_red(),
            ": ".bright_white(),
            self.error.ty.bright_white(),
        )?;

        let annotations = [Annotation {
            mode: Mode::Error,
            text: format!("{}", self.error.ty.inline_display()),
            note: "".to_string(),
            highlight: self.error.range.clone(),
            visible: self.error.range.clone(),
        }];

        write!(
            f,
            "{}",
            display_annotations(self.file_name, self.source, &annotations)
        )?;
        Ok(())
    }
}
