use crate::token::Token;

/// A list of at least one sub-grammar, separated by an individual token.
///
/// # Examples
///
/// [SeparatedList1]<`A` `,`> matches:
///  - `A`
///  - `A, A`
///  - `A, A, A`
///
/// Does not match:
///  - ` ` (empty input)
///  - `,`
///  - `, A`
///  - `A, A,`
#[derive(Debug, Clone)]
pub struct SeparatedList1<'s, T> {
    pub items: Vec<(T, &'s Token<'s>)>,
    pub last_item: Box<T>,
}

/// A list of zero or more sub-grammars, separated by an individual token.
///
/// # Examples
///
/// [SeparatedList0]<`A` `,`> matches:
///  - ` ` (empty input)
///  - `A`
///  - `A, A`
///
/// Does not match:
///  - `,`
///  - `, A`
///  - `A, A,`
pub type SeparatedList0<'s, T> = Option<SeparatedList1<'s, T>>;

/// A list of at least one sub-grammar, separated by an individual token, with an optional trailing
/// separator.
///
/// # Examples
///
/// [SeparatedListTrailing1]<`A` `,`> matches:
///  - `A`
///  - `A,`
///  - `A, A`
///  - `A, A,`
///
/// Does not match:
///  - ` ` (empty input)
///  - `,`
///  - `, A`
#[derive(Debug, Clone)]
pub struct SeparatedListTrailing1<'s, T> {
    pub items: Vec<(T, &'s Token<'s>)>,
    pub last_item: Box<T>,
    pub trailing: Option<&'s Token<'s>>,
}

/// A list of zero or more sub-grammars, separated by an individual token, with an optional trailing
/// separator if the list has at least one sub-grammar.
///
/// # Examples
///
/// [SeparatedListTrailing0]<`A` `,`> matches:
///  - ` ` (empty input)
///  - `A`
///  - `A,`
///  - `A, A`
///  - `A, A,`
///
/// Does not match:
///  - `,`
///  - `, A`
pub type SeparatedListTrailing0<'s, T> = Option<SeparatedListTrailing1<'s, T>>;

impl<'s, T> SeparatedList1<'s, T> {
    /// Pushes an item to the list.
    pub fn push(&mut self, last_separator: &'s Token<'s>, item: T) {
        self.items.push((
            std::mem::replace(self.last_item.as_mut(), item),
            last_separator,
        ));
    }

    /// Adds a trailing separator, converting the list to a [`SeparatedListTrailing1`].
    pub fn into_trailing(self, trailing: Option<&'s Token<'s>>) -> SeparatedListTrailing1<'s, T> {
        SeparatedListTrailing1 {
            items: self.items,
            last_item: self.last_item,
            trailing,
        }
    }
}
