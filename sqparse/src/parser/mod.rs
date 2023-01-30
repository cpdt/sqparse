use crate::ast::{Expression, Statement};
use crate::parser::statement::parse_document;
use crate::token::TokenType;
use crate::{ParseError, Token};
use nom::{Compare, CompareResult, Finish, InputLength, InputTake, Offset, Slice};
use std::ops::RangeTo;
use nom::combinator::{complete, eof};
use nom::sequence::pair;
use crate::parser::expression::expression;

mod expression;
mod shared;
mod statement;
mod ty;

pub type IResult<I, O> = nom::IResult<I, O, ParseError>;

#[derive(Debug, Clone, Copy)]
pub struct TokenList<'s> {
    tokens: &'s [Token<'s>],
    offset: usize,
}

impl<'s> TokenList<'s> {
    pub fn new(tokens: &'s [Token<'s>]) -> Self {
        TokenList { tokens, offset: 0 }
    }

    pub fn get_offset(self) -> usize {
        self.offset
    }

    pub fn first(self) -> Option<&'s Token<'s>> {
        self.tokens.first()
    }

    pub fn always_first(self) -> &'s Token<'s> {
        &self.tokens[0]
    }

    pub fn end_of_line(self) -> bool {
        match self.tokens.last() {
            Some(token) => token.new_line.is_some(),
            None => false,
        }
    }

    pub fn is_empty(self) -> bool {
        self.tokens.is_empty()
    }
}

impl<'s> InputLength for TokenList<'s> {
    fn input_len(&self) -> usize {
        self.tokens.len()
    }
}

impl<'s> InputTake for TokenList<'s> {
    fn take(&self, count: usize) -> Self {
        TokenList {
            tokens: &self.tokens[0..count],
            offset: self.offset,
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tokens.split_at(count);
        (
            TokenList {
                tokens: suffix,
                offset: self.offset + count,
            },
            TokenList {
                tokens: prefix,
                offset: self.offset,
            },
        )
    }
}

impl<'s, 't> Compare<TokenType<'t>> for TokenList<'s> {
    fn compare(&self, ty: TokenType<'t>) -> CompareResult {
        match self.tokens.first() {
            Some(first_token) if first_token.ty == ty => CompareResult::Ok,
            Some(_) => CompareResult::Error,
            None => CompareResult::Incomplete,
        }
    }

    fn compare_no_case(&self, t: TokenType<'t>) -> CompareResult {
        // todo: special handling needed?
        self.compare(t)
    }
}

impl<'s> Offset for TokenList<'s> {
    fn offset(&self, second: &Self) -> usize {
        second.offset - self.offset
    }
}

impl<'s> Slice<RangeTo<usize>> for TokenList<'s> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        TokenList {
            tokens: &self.tokens[range],
            offset: self.offset,
        }
    }
}

impl<'s> InputLength for TokenType<'s> {
    fn input_len(&self) -> usize {
        1
    }
}

pub fn parse<'s>(tokens: &'s [Token<'s>]) -> Result<Vec<Statement<'s>>, ParseError> {
    let (_, statements) = parse_document(TokenList::new(tokens)).finish()?;
    Ok(statements)
}

pub fn parse_expression<'s>(tokens: &'s [Token<'s>]) -> Result<Expression<'s>, ParseError> {
    let (_, expression) = complete(expression)(TokenList::new(tokens)).finish()?;
    Ok(expression)
}
