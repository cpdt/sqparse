use crate::lexer_error::LexerErrorType;
use crate::token::{
    Comment, LiteralBase, LiteralToken, StringToken, TerminalToken, Token, TokenLine, TokenType,
};
use crate::{Flavor, LexerError};
use lazy_static::lazy_static;
use lexical::NumberFormatBuilder;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ParseStr<'s> {
    val: &'s str,
    offset: usize,
}

impl<'s> ParseStr<'s> {
    pub fn new(val: &'s str) -> ParseStr<'s> {
        ParseStr { val, offset: 0 }
    }

    pub fn as_str(self) -> &'s str {
        self.val
    }

    pub fn from(self, idx: usize) -> ParseStr<'s> {
        ParseStr {
            val: &self.val[idx..],
            offset: self.offset + idx,
        }
    }

    pub fn end(self) -> ParseStr<'static> {
        ParseStr {
            val: "",
            offset: self.offset + self.val.len(),
        }
    }

    pub fn start_offset(self) -> usize {
        self.offset
    }

    pub fn end_offset(self) -> usize {
        self.offset + self.val.len()
    }

    // Pass None to split at the end of the string
    pub fn split_at(self, mid: impl Into<Option<usize>>) -> (&'s str, ParseStr<'s>) {
        match mid.into() {
            Some(mid) => (&self.val[..mid], self.from(mid)),
            None => (self.val, self.end()),
        }
    }

    // trims whitespace excluding newlines
    pub fn trim_start(self) -> ParseStr<'s> {
        match self.val.find(|c: char| c == '\n' || !c.is_whitespace()) {
            Some(start_index) => self.from(start_index),
            None => self.end(),
        }
    }

    pub fn strip_prefix(self, prefix: &str) -> Option<ParseStr<'s>> {
        self.val.strip_prefix(prefix).map(|val| ParseStr {
            val,
            offset: self.offset + prefix.len(),
        })
    }
}

impl<'s> Deref for ParseStr<'s> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.val
    }
}

fn is_identifier_char(c: char) -> bool {
    c == '_' || c.is_ascii_alphanumeric()
}

fn try_identifier(val: ParseStr) -> Option<(&str, ParseStr)> {
    let first_char = val.chars().next()?;
    if first_char != '_' && !first_char.is_ascii_alphabetic() {
        return None;
    }

    Some(val.split_at(val.find(|c: char| !is_identifier_char(c))))
}

fn identifier_as_token(identifier: &str, flavor: Flavor) -> Option<TerminalToken> {
    lazy_static! {
        static ref IDENTIFIERS_MAP: HashMap<&'static str, TerminalToken> =
            TerminalToken::IDENTIFIERS
                .iter()
                .map(|(token, token_val)| (*token_val, *token))
                .collect();
    }

    IDENTIFIERS_MAP
        .get(identifier)
        .cloned()
        .filter(|token| token.is_supported(flavor))
}

fn try_symbol_token(val: ParseStr, flavor: Flavor) -> Option<(TerminalToken, ParseStr)> {
    TerminalToken::SYMBOLS
        .iter()
        .filter(|(token, _)| token.is_supported(flavor))
        .find(|(_, token_val)| val.starts_with(token_val))
        .map(|(token, token_val)| (*token, val.from(token_val.len())))
}

fn try_string_val(val: ParseStr, delimiter: char) -> Result<Option<(&str, ParseStr)>, LexerError> {
    if !val.starts_with(delimiter) {
        return Ok(None);
    }

    let val = val.from(1);

    // Scan the string looking for an escape sequence or end of string
    let mut char_indices = val.char_indices();
    loop {
        let (next_index, next_char) = match char_indices.next() {
            Some(val) => val,
            None => {
                return Err(LexerError::new(
                    val.end_offset(),
                    LexerErrorType::EndOfInputInsideString,
                ));
            }
        };

        if next_char == '\n' {
            return Err(LexerError::new(
                val.start_offset() + next_index,
                LexerErrorType::EndOfLineInsideString,
            ));
        }

        if next_char == '\\' {
            // Skip the next char and continue
            char_indices.next();
            continue;
        }

        if next_char == delimiter {
            // todo: might panic if this was the last char of the input
            return Ok(Some((
                &val.as_str()[..next_index],
                val.from(next_index + 1),
            )));
        }
    }
}

fn try_string(val: ParseStr) -> Result<Option<(StringToken, ParseStr)>, LexerError> {
    if val.starts_with('@') {
        return Ok(try_string_val(val.from(1), '"')?
            .map(|(val, remaining)| (StringToken::Verbatim(val), remaining)));
    }

    if val.starts_with('$') {
        return Ok(try_string_val(val.from(1), '"')?
            .map(|(val, remaining)| (StringToken::Asset(val), remaining)));
    }

    Ok(try_string_val(val, '"')?.map(|(val, remaining)| (StringToken::Literal(val), remaining)))
}

fn starts_with_octal(val: &str) -> bool {
    let mut chars = val.chars();
    match chars.next() {
        Some('0') => {}
        Some(_) | None => return false,
    };
    match chars.next() {
        Some(digit) => digit.is_ascii_digit(),
        None => false,
    }
}

fn try_number(val: ParseStr) -> Result<Option<(LiteralToken, ParseStr)>, LexerError> {
    let starts_with_digit = val.starts_with(|c: char| c.is_ascii_digit());

    if starts_with_digit {
        let (base, remaining) = if val.starts_with("0x") {
            (LiteralBase::Decimal, val.from(2))
        } else if starts_with_octal(val.as_str()) {
            (LiteralBase::Octal, val.from(1))
        } else {
            (LiteralBase::Decimal, val)
        };

        const DECIMAL_FORMAT: u128 = NumberFormatBuilder::decimal();
        const OCTAL_FORMAT: u128 = NumberFormatBuilder::octal();
        const HEXADECIMAL_FORMAT: u128 = NumberFormatBuilder::hexadecimal();

        let parse_result = match base {
            LiteralBase::Decimal => lexical::parse_partial_with_options::<i64, _, DECIMAL_FORMAT>(
                remaining.as_str(),
                &lexical::ParseIntegerOptions::new(),
            ),
            LiteralBase::Octal => lexical::parse_partial_with_options::<i64, _, OCTAL_FORMAT>(
                remaining.as_str(),
                &lexical::ParseIntegerOptions::new(),
            ),
            LiteralBase::Hexadecimal => {
                lexical::parse_partial_with_options::<i64, _, HEXADECIMAL_FORMAT>(
                    remaining.as_str(),
                    &lexical::ParseIntegerOptions::new(),
                )
            }
        };

        let (int_val, remaining) = match parse_result {
            Err(_) | Ok((_, 0)) => return Ok(None),
            Ok((parsed_val, len)) => (LiteralToken::Int(parsed_val, base), remaining.from(len)),
        };

        if !remaining.starts_with('.') {
            return Ok(Some((int_val, remaining)));
        }
    }

    let starts_with_dot = val.starts_with('.');
    if starts_with_digit || starts_with_dot {
        match lexical::parse_partial::<f64, _>(val.as_str()) {
            Err(_) => Ok(None),
            Ok((_, 1)) if starts_with_dot => Ok(None),
            Ok((parsed_val, len)) => {
                // The Squirrel lexer is a bit cursed and parses but ignores any [.0-9] chars after
                // a floating point number, so we need to skip those here
                let val = val.from(len);
                let float_end = val
                    .find(|c: char| c != '.' && !c.is_ascii_digit())
                    .unwrap_or_else(|| val.len());
                Ok(Some((LiteralToken::Float(parsed_val), val.from(float_end))))
            }
        }
    } else {
        Ok(None)
    }
}

fn try_literal(val: ParseStr) -> Result<Option<(LiteralToken, ParseStr)>, LexerError> {
    if let Some(num_val) = try_number(val)? {
        return Ok(Some(num_val));
    }
    if let Some((char_val, remaining)) = try_string_val(val, '\'')? {
        return Ok(Some((LiteralToken::Char(char_val), remaining)));
    }
    if let Some((str_val, remaining)) = try_string(val)? {
        return Ok(Some((LiteralToken::String(str_val), remaining)));
    }
    Ok(None)
}

fn try_token(val: ParseStr, flavor: Flavor) -> Result<Option<(TokenType, ParseStr)>, LexerError> {
    if let Some((literal_val, remaining)) = try_literal(val)? {
        return Ok(Some((TokenType::Literal(literal_val), remaining)));
    }
    if let Some((terminal_val, remaining)) = try_symbol_token(val, flavor) {
        return Ok(Some((TokenType::Terminal(terminal_val), remaining)));
    }
    if let Some((identifier_val, remaining)) = try_identifier(val) {
        let token_ty = match identifier_as_token(identifier_val, flavor) {
            Some(terminal) => TokenType::Terminal(terminal),
            None => TokenType::Identifier(identifier_val),
        };
        return Ok(Some((token_ty, remaining)));
    }

    Ok(None)
}

fn get_rest_of_line(val: ParseStr) -> (&str, ParseStr) {
    val.split_at(val.find('\n'))
}

fn try_comment(val: ParseStr) -> Result<Option<(Comment, ParseStr)>, LexerError> {
    if let Some(val) = val.strip_prefix("/*") {
        return match val.find("*/") {
            Some(end_index) => Ok(Some((
                Comment::MultiLine(&val.as_str()[..end_index]),
                val.from(end_index + 2),
            ))),
            None => Err(LexerError::new(
                val.end_offset(),
                LexerErrorType::EndOfInputInsideComment,
            )),
        };
    }

    if val.starts_with('#') {
        let (comment_val, remaining_val) = get_rest_of_line(val.from(1));
        return Ok(Some((Comment::ScriptStyle(comment_val), remaining_val)));
    }

    if val.starts_with("//") {
        let (comment_val, remaining_val) = get_rest_of_line(val.from(2));
        return Ok(Some((Comment::SingleLine(comment_val), remaining_val)));
    }

    Ok(None)
}

fn push_line_comments<'s>(
    tokens: &mut Vec<Token<'s>>,
    comment_lines: &mut Vec<TokenLine<'s>>,
    comments: Vec<Comment<'s>>,
) {
    match tokens.last_mut() {
        Some(token) if token.new_line.is_none() => token.new_line = Some(TokenLine { comments }),
        _ => comment_lines.push(TokenLine { comments }),
    };
}

pub fn tokenize(val: &str, flavor: Flavor) -> Result<Vec<Token>, LexerError> {
    let mut remaining_val = ParseStr::new(val);
    let mut tokens: Vec<Token> = Vec::new();
    let mut comment_lines = Vec::new();
    let mut current_comments = Vec::new();

    while !remaining_val.is_empty() {
        remaining_val = remaining_val.trim_start();

        if remaining_val.starts_with('\n') {
            push_line_comments(
                &mut tokens,
                &mut comment_lines,
                std::mem::take(&mut current_comments),
            );
            remaining_val = remaining_val.from(1);
            continue;
        }

        if let Some((comment, remaining)) = try_comment(remaining_val)? {
            assert_ne!(remaining_val.start_offset(), remaining.start_offset());

            current_comments.push(comment);
            remaining_val = remaining;
            continue;
        }

        match try_token(remaining_val, flavor)? {
            Some((token_ty, remaining)) => {
                assert_ne!(remaining_val.start_offset(), remaining.start_offset());

                let comments = std::mem::take(&mut current_comments);
                let before_lines = std::mem::take(&mut comment_lines);
                tokens.push(Token {
                    ty: token_ty,
                    range: remaining_val.start_offset()..remaining.start_offset(),

                    comments,
                    before_lines,

                    // will be filled in later when a \n is encountered
                    new_line: None,
                });
                remaining_val = remaining;
            }
            None => {
                return Err(LexerError::new(
                    remaining_val.start_offset(),
                    LexerErrorType::InvalidInput,
                ));
            }
        }
    }

    push_line_comments(&mut tokens, &mut comment_lines, current_comments);
    if !comment_lines.is_empty() {
        tokens.push(Token {
            ty: TokenType::Empty,
            range: val.len()..val.len(),
            comments: Vec::new(),
            before_lines: comment_lines,
            new_line: None,
        });
    }
    Ok(tokens)
}
