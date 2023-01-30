use std::sync::Arc;
use sqparse::ast::{Expression, LiteralExpression};
use sqparse::{Flavor, Token};
use sqparse::token::{LiteralToken, TokenType};
use crate::config::Format;
use crate::writer::Writer;

pub fn mock_format() -> Format {
    Format {
        column_limit: 20,

        indent: "    ".to_string(),
        indent_columns: 4,

        spaces_in_expr_brackets: false,

        array_spaces: false,
        array_multiline_commas: false,
        array_multiline_trailing_commas: false,
        array_singleline_trailing_commas: false,
    }
}

pub fn mock_token(ty: TokenType) -> Token {
    Token {
        ty,
        range: 0..0,
        comments: Vec::new(),
        before_lines: Vec::new(),
        new_line: None,
    }
}

pub fn tokens(val: &str) -> Vec<Token> {
    let mut tokens = sqparse::tokenize(val, Flavor::SquirrelRespawn).unwrap();
    tokens.push(Token {
        ty: TokenType::Empty,
        range: 0..0,
        comments: Vec::new(),
        before_lines: Vec::new(),
        new_line: None,
    });
    tokens
}

pub fn expr<'s>(tokens: &'s [Token<'s>]) -> Expression<'s> {
    sqparse::parse_expression(tokens).unwrap()
}

pub fn test_write<F: FnOnce(Writer) -> Option<Writer>>(f: F) -> String {
    f(Writer::new(Arc::new(mock_format()))).unwrap().to_string()
}

pub fn test_write_columns<F: FnOnce(Writer) -> Option<Writer>>(column_limit: usize, f: F) -> String {
    let format = Format {
        column_limit,
        ..mock_format()
    };
    f(Writer::new(Arc::new(format))).unwrap().to_string()
}

pub fn test_write_format<F: FnOnce(Writer) -> Option<Writer>>(format: Format, f: F) -> String {
    f(Writer::new(Arc::new(format))).unwrap().to_string()
}
