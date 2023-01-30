use crate::lexer::TokenItem;

#[derive(Debug, Clone, Copy)]
pub struct TokenList<'s> {
    tokens: &'s [TokenItem<'s>],
    index: usize,
}

impl<'s> TokenList<'s> {
    pub fn new(tokens: &'s [TokenItem<'s>]) -> Self {
        TokenList { tokens, index: 0 }
    }

    pub fn previous(self) -> Option<&'s TokenItem<'s>> {
        if self.index > 0 {
            self.tokens.get(self.index - 1)
        } else {
            None
        }
    }

    pub fn next(self) -> Option<&'s TokenItem<'s>> {
        self.tokens.get(self.index)
    }

    pub fn is_ended(self) -> bool {
        self.index == self.tokens.len()
    }

    pub fn start_index(self) -> usize {
        self.index
    }

    pub fn is_newline(self) -> bool {
        self.previous()
            .map(|item| item.token.new_line.is_some())
            .unwrap_or(false)
    }

    pub fn split_first(self) -> Option<(TokenList<'s>, &'s TokenItem<'s>)> {
        self.next().map(|first| {
            (
                TokenList {
                    tokens: self.tokens,
                    index: self.index + 1,
                },
                first,
            )
        })
    }

    pub fn split_at(self, index: usize) -> (TokenList<'s>, TokenList<'s>) {
        assert!(index >= self.index);
        (
            TokenList {
                tokens: &self.tokens[..index],
                index: self.index,
            },
            TokenList {
                tokens: self.tokens,
                index,
            },
        )
    }
}
