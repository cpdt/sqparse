use crate::parser::error::TokenAffinity;
use crate::parser::token_list::TokenList;
use crate::parser::token_list_ext::TokenListExt;
use crate::parser::ParseResult;
use crate::{ContextType, ParseError, ParseErrorType};
use std::ops::Range;

pub trait ParseResultExt<'s, T>: Sized {
    fn into_parse_result(self) -> ParseResult<'s, T>;

    fn maybe(self, tokens: TokenList<'s>) -> ParseResult<'s, Option<T>> {
        match self.into_parse_result() {
            Ok((tokens, val)) => Ok((tokens, Some(val))),
            Err(err) if err.is_fatal => Err(err),
            Err(_) => Ok((tokens, None)),
        }
    }

    fn not_definite(self) -> ParseResult<'s, T> {
        self.into_parse_result().map_err(|err| err.into_non_fatal())
    }

    fn definite(self) -> ParseResult<'s, T> {
        self.into_parse_result().map_err(|err| err.into_fatal())
    }

    fn not_line_ending(self) -> ParseResult<'s, T> {
        let (tokens, val) = self.into_parse_result()?;
        if tokens.is_newline() {
            Err(tokens.error(ParseErrorType::IllegalLineBreak))
        } else {
            Ok((tokens, val))
        }
    }

    fn map_val<B, F: FnOnce(T) -> B>(self, map: F) -> ParseResult<'s, B> {
        self.into_parse_result().map(|(tokens, a)| (tokens, map(a)))
    }

    fn or_try<F: FnOnce() -> ParseResult<'s, T>>(self, f: F) -> ParseResult<'s, T> {
        match self.into_parse_result() {
            Ok((tokens, val)) => Ok((tokens, val)),
            Err(err) if err.is_fatal => Err(err),
            Err(_) => f(),
        }
    }

    fn or_error<F: FnOnce() -> ParseError>(self, f: F) -> ParseResult<'s, T> {
        self.or_try(|| Err(f()))
    }

    fn with_context(self, ty: ContextType, range: Range<usize>) -> ParseResult<'s, T> {
        self.replace_context(ContextType::Span, ty, range)
    }

    fn with_context_from(self, ty: ContextType, tokens: TokenList) -> ParseResult<'s, T> {
        self.replace_context_from(ContextType::Span, ty, tokens)
    }

    fn replace_context(
        self,
        from_ty: ContextType,
        to_ty: ContextType,
        range: Range<usize>,
    ) -> ParseResult<'s, T> {
        self.into_parse_result()
            .map_err(|err| err.replace_context(from_ty, to_ty, range, TokenAffinity::Inline))
    }

    fn replace_context_from(
        self,
        from_ty: ContextType,
        to_ty: ContextType,
        tokens: TokenList,
    ) -> ParseResult<'s, T> {
        let start_index = tokens.start_index();
        self.into_parse_result().map_err(|err| {
            let range = start_index..err.token_index;
            err.replace_context(from_ty, to_ty, range, TokenAffinity::Before)
        })
    }

    fn determines<B, F: FnOnce(TokenList<'s>, T) -> ParseResult<'s, B>>(
        self,
        f: F,
    ) -> ParseResult<'s, B> {
        let (tokens, a) = self.into_parse_result().not_definite()?;
        f(tokens, a).definite()
    }

    fn opens<
        Close,
        Out,
        FClose: Fn(TokenList<'s>) -> ParseResult<'s, Close>,
        FInner: FnOnce(TokenList<'s>, T, Close) -> ParseResult<'s, Out>,
    >(
        self,
        context: ContextType,
        close: FClose,
        inner: FInner,
    ) -> ParseResult<'s, Out> {
        let (tokens, open_val) = self.into_parse_result()?;
        let close_index = tokens.previous().unwrap().close_index.unwrap();
        let span_range = (tokens.start_index() - 1)..(close_index + 1);

        let (inner_tokens, outer_tokens) = tokens.split_at(close_index);
        let (outer_tokens, close_val) =
            close(outer_tokens).with_context(context, span_range.clone())?;
        let (inner_tokens, value) =
            inner(inner_tokens, open_val, close_val).with_context(context, span_range.clone())?;

        if !inner_tokens.is_ended() {
            let mut remaining_tokens = inner_tokens;

            // Hack to trigger an error with the close parser, since the inner parser did not
            // consume all of the input.
            loop {
                let (new_tokens, _) =
                    close(remaining_tokens).with_context(context, span_range.clone())?;

                // If parsing succeeded with no input tokens, something has gone wrong and we don't
                // want to loop forever.
                assert!(!remaining_tokens.is_ended());

                remaining_tokens = new_tokens;
            }
        }

        Ok((outer_tokens, value))
    }

    fn determines_and_opens<
        Close,
        Out,
        FClose: Fn(TokenList<'s>) -> ParseResult<'s, Close>,
        FInner: FnOnce(TokenList<'s>, T, Close) -> ParseResult<'s, Out>,
    >(
        self,
        context: ContextType,
        close: FClose,
        inner: FInner,
    ) -> ParseResult<'s, Out> {
        let (tokens, open_val) = self.into_parse_result().not_definite()?;
        Ok((tokens, open_val))
            .opens(context, close, inner)
            .definite()
    }
}

impl<'s, T> ParseResultExt<'s, T> for ParseResult<'s, T> {
    #[inline]
    fn into_parse_result(self) -> ParseResult<'s, T> {
        self
    }
}
