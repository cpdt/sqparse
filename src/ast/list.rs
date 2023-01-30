use crate::token::Token;

// A list of at least one value, separated by tokens.
// E.g. A `,` B `,` C `,` D
#[derive(Debug, Clone)]
pub struct SeparatedList1<'s, T> {
    pub items: Vec<(T, &'s Token<'s>)>,
    pub last_item: Box<T>,
}

pub type SeparatedList0<'s, T> = Option<SeparatedList1<'s, T>>;

// A list of at least one value separated by tokens, optionally with a trailing separator.
// E.g. A `,` B `,` C `,` D `,`
#[derive(Debug, Clone)]
pub struct SeparatedListTrailing1<'s, T> {
    pub items: Vec<(T, &'s Token<'s>)>,
    pub last_item: Box<T>,
    pub trailing: Option<&'s Token<'s>>,
}

pub type SeparatedListTrailing0<'s, T> = Option<SeparatedListTrailing1<'s, T>>;

impl<'s, T> SeparatedList1<'s, T> {
    pub fn push(&mut self, last_separator: &'s Token<'s>, item: T) {
        self.items.push((
            std::mem::replace(self.last_item.as_mut(), item),
            last_separator,
        ));
    }

    pub fn into_trailing(self, trailing: Option<&'s Token<'s>>) -> SeparatedListTrailing1<'s, T> {
        SeparatedListTrailing1 {
            items: self.items,
            last_item: self.last_item,
            trailing,
        }
    }
}
