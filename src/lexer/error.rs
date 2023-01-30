use crate::display_error;
use crate::token::TokenType;
use std::ops::Range;

#[derive(Debug, Clone, Copy)]
pub enum LexerErrorType<'s> {
    EndOfInputInsideString,
    EndOfLineInsideString,
    EndOfInputInsideComment,
    InvalidInput,
    UnmatchedOpener {
        open: TokenType<'s>,
        close: TokenType<'s>,
    },
}

#[derive(Debug, Clone)]
pub struct LexerError<'s> {
    pub range: Range<usize>,
    pub ty: LexerErrorType<'s>,
}

impl<'s> LexerError<'s> {
    pub fn new(range: Range<usize>, ty: LexerErrorType<'s>) -> Self {
        LexerError { range, ty }
    }

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
