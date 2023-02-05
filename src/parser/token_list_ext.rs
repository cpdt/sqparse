use crate::ast::{SeparatedList1, SeparatedListTrailing0, SeparatedListTrailing1};
use crate::parser::error::TokenAffinity;
use crate::parser::parse_result_ext::ParseResultExt;
use crate::parser::token_list::TokenList;
use crate::parser::ParseResult;
use crate::token::{TerminalToken, Token, TokenType};
use crate::{ParseError, ParseErrorType, TokenItem};

pub trait TokenListExt<'s>: Sized {
    fn into_token_list(self) -> TokenList<'s>;

    fn error(self, ty: ParseErrorType) -> ParseError {
        let tokens = self.into_token_list();
        let affinity = if tokens.is_ended() {
            TokenAffinity::Before
        } else {
            TokenAffinity::Inline
        };
        ParseError::new(ty, tokens.start_index(), affinity)
    }

    fn error_before(self, ty: ParseErrorType) -> ParseError {
        ParseError::new(
            ty,
            self.into_token_list().start_index(),
            TokenAffinity::Before,
        )
    }

    fn ended_or<T>(self, res: ParseResult<'s, T>) -> ParseResult<'s, T> {
        if self.into_token_list().is_ended() {
            res
        } else {
            res.definite()
        }
    }

    fn empty(self) -> Option<(TokenList<'s>, &'s Token<'s>)> {
        let tokens = self.into_token_list();
        if let Some((tokens, item)) = tokens.split_first() {
            if let TokenType::Empty = item.token.ty {
                return Some((tokens, &item.token));
            }
        }

        None
    }

    fn terminal_item(self, terminal: TerminalToken) -> ParseResult<'s, &'s TokenItem<'s>> {
        let tokens = self.into_token_list();
        if let Some((tokens, item)) = tokens.split_first() {
            if let TokenType::Terminal(received) = item.token.ty {
                if received == terminal {
                    return Ok((tokens, item));
                }
            }
        }

        Err(tokens.error(ParseErrorType::ExpectedTerminal(terminal)))
    }

    fn terminal(self, terminal: TerminalToken) -> ParseResult<'s, &'s Token<'s>> {
        let (tokens, item) = self.terminal_item(terminal)?;
        Ok((tokens, &item.token))
    }

    fn terminal2(
        self,
        terminal_1: TerminalToken,
        terminal_2: TerminalToken,
    ) -> ParseResult<'s, (&'s Token<'s>, &'s Token<'s>)> {
        let (tokens, token_1) = self.terminal(terminal_1)?;
        let (tokens, token_2) = tokens.terminal(terminal_2)?;

        if token_1.range.end == token_2.range.start {
            return Ok((tokens, (token_1, token_2)));
        }

        Err(tokens.error(ParseErrorType::ExpectedCompound2(terminal_1, terminal_2)))
    }

    fn terminal3(
        self,
        terminal_1: TerminalToken,
        terminal_2: TerminalToken,
        terminal_3: TerminalToken,
    ) -> ParseResult<'s, (&'s Token<'s>, &'s Token<'s>, &'s Token<'s>)> {
        let (tokens, token_1) = self.terminal(terminal_1)?;
        let (tokens, token_2) = tokens.terminal(terminal_2)?;
        let (tokens, token_3) = tokens.terminal(terminal_3)?;

        if token_1.range.end == token_2.range.start && token_2.range.end == token_3.range.start {
            return Ok((tokens, (token_1, token_2, token_3)));
        }

        Err(tokens.error(ParseErrorType::ExpectedCompound3(
            terminal_1, terminal_2, terminal_3,
        )))
    }

    fn many<T, F: FnMut(TokenList<'s>) -> ParseResult<'s, T>>(
        self,
        mut parse_item: F,
    ) -> ParseResult<'s, Vec<T>> {
        let mut tokens = self.into_token_list();
        let mut values = Vec::new();
        while let (next_tokens, Some(item)) = parse_item(tokens).maybe(tokens)? {
            tokens = next_tokens;
            values.push(item);
        }
        Ok((tokens, values))
    }

    fn many_until<
        T,
        FCond: FnMut(TokenList<'s>) -> bool,
        FItem: FnMut(TokenList<'s>) -> ParseResult<'s, T>,
    >(
        self,
        mut cond: FCond,
        mut parse_item: FItem,
    ) -> ParseResult<'s, Vec<T>> {
        let mut tokens = self.into_token_list();
        let mut values = Vec::new();
        while !cond(tokens) {
            let (new_tokens, item) = parse_item(tokens)?;
            tokens = new_tokens;
            values.push(item);
        }
        Ok((tokens, values))
    }

    fn many_until_ended<T, FItem: FnMut(TokenList<'s>) -> ParseResult<'s, T>>(
        self,
        parse_item: FItem,
    ) -> ParseResult<'s, Vec<T>> {
        self.many_until(|tokens| tokens.is_ended(), parse_item)
    }

    fn separated_list1<
        T,
        FItem: FnMut(TokenList<'s>) -> ParseResult<'s, T>,
        FSeparator: FnMut(TokenList<'s>) -> ParseResult<'s, &'s Token<'s>>,
    >(
        self,
        mut parse_item: FItem,
        mut parse_separator: FSeparator,
    ) -> ParseResult<'s, SeparatedList1<'s, T>> {
        let (mut tokens, first_item) = parse_item(self.into_token_list())?;
        let mut list = SeparatedList1 {
            items: Vec::new(),
            last_item: Box::new(first_item),
        };

        while let (next_tokens, Some(separator)) = parse_separator(tokens).maybe(tokens)? {
            let (next_tokens, next_item) = parse_item(next_tokens)?;
            tokens = next_tokens;
            list.push(separator, next_item);
        }

        Ok((tokens, list))
    }

    fn separated_list_trailing1<
        T,
        FItem: FnMut(TokenList<'s>) -> ParseResult<'s, T>,
        FSeparator: FnMut(TokenList<'s>) -> ParseResult<'s, &'s Token<'s>>,
    >(
        self,
        mut parse_item: FItem,
        parse_separator: FSeparator,
    ) -> ParseResult<'s, SeparatedListTrailing1<'s, T>> {
        let (tokens, first_item) = parse_item(self.into_token_list())?;
        tokens.separated_list_trailing1_init(first_item, parse_item, parse_separator)
    }

    fn separated_list_trailing0<
        T,
        FItem: FnMut(TokenList<'s>) -> ParseResult<'s, T>,
        FSeparator: FnMut(TokenList<'s>) -> ParseResult<'s, &'s Token<'s>>,
    >(
        self,
        mut parse_item: FItem,
        parse_separator: FSeparator,
    ) -> ParseResult<'s, SeparatedListTrailing0<'s, T>> {
        let tokens = self.into_token_list();
        let (tokens, Some(first_item)) = parse_item(tokens).maybe(tokens)? else {
            return Ok((tokens, None));
        };
        tokens
            .separated_list_trailing1_init(first_item, parse_item, parse_separator)
            .map_val(Some)
    }

    fn separated_list_trailing1_init<
        T,
        FItem: FnMut(TokenList<'s>) -> ParseResult<'s, T>,
        FSeparator: FnMut(TokenList<'s>) -> ParseResult<'s, &'s Token<'s>>,
    >(
        self,
        first_item: T,
        mut parse_item: FItem,
        mut parse_separator: FSeparator,
    ) -> ParseResult<'s, SeparatedListTrailing1<'s, T>> {
        let mut tokens = self.into_token_list();
        let mut list = SeparatedList1 {
            items: Vec::new(),
            last_item: Box::new(first_item),
        };

        while let (next_tokens, Some(separator)) = parse_separator(tokens).maybe(tokens)? {
            let (next_tokens, Some(next_item)) = parse_item(next_tokens).maybe(next_tokens)? else {
                return Ok((next_tokens, list.into_trailing(Some(separator))));
            };
            tokens = next_tokens;
            list.push(separator, next_item);
        }

        Ok((tokens, list.into_trailing(None)))
    }
}

impl<'s> TokenListExt<'s> for TokenList<'s> {
    fn into_token_list(self) -> TokenList<'s> {
        self
    }
}
