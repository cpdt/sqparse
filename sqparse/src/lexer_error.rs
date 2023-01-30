use crate::error_display::error;
use std::fmt::Formatter;

#[derive(Debug, Clone, Copy)]
pub enum LexerErrorType {
    EndOfInputInsideString,
    EndOfLineInsideString,
    EndOfInputInsideComment,
    InvalidInput,
}

#[derive(Debug, Clone, Copy)]
pub struct LexerError {
    char_offset: usize,
    ty: LexerErrorType,
}

impl LexerError {
    pub fn new(char_offset: usize, ty: LexerErrorType) -> Self {
        LexerError { char_offset, ty }
    }

    pub fn display<'s>(&'s self, source: &'s str) -> Display<'s> {
        Display {
            error: self,
            source,
        }
    }
}

pub struct Display<'s> {
    error: &'s LexerError,
    source: &'s str,
}

impl<'s> std::fmt::Display for Display<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let range = self.error.char_offset..(self.error.char_offset + 1);
        write!(f, "{} {}", error(range, self.source), self.error.ty)
    }
}

impl std::fmt::Display for LexerErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerErrorType::EndOfInputInsideString => {
                write!(f, "file ended in the middle of a string")
            }
            LexerErrorType::EndOfLineInsideString => {
                write!(f, "strings cannot span multiple lines")
            }
            LexerErrorType::EndOfInputInsideComment => {
                write!(f, "file ended in the middle of a comment")
            }
            LexerErrorType::InvalidInput => write!(f, "not sure what this is"),
        }
    }
}
