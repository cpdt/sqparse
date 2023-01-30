use sqparse::ast::Identifier;
use sqparse::Token;
use crate::combinators::{cond_or, opt, opt_or, tag};
use crate::token::{discard_token, token};
use crate::writer::Writer;

pub fn identifier<'s>(identifier: &'s Identifier<'s>) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    token(identifier.token)
}

pub fn token_or_tag<'s>(t: Option<&'s Token<'s>>, fallback_tag: &'static str) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    opt_or(t, token, tag(fallback_tag))
}

pub fn optional_separator<'s>(is_required: bool, separator_token: Option<&'s Token<'s>>, fallback_tag: &'static str) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    cond_or(
        is_required,
        token_or_tag(separator_token, fallback_tag),
        opt(separator_token, discard_token),
    )
}
