use crate::lexer::token_iter::TokenIter;
use crate::token::{TerminalToken, Token, TokenType};
use crate::Flavor;
use std::collections::VecDeque;

mod comment;
mod error;
mod identifier;
mod literal;
mod parse_str;
mod symbol;
mod token_iter;

pub use self::error::{LexerError, LexerErrorType};

#[derive(Debug, Clone, PartialEq)]
pub struct TokenItem<'s> {
    pub token: Token<'s>,
    pub close_index: Option<usize>,
}

// Returns the token that closes a tree, if the provided token is a valid opening token.
fn closing_token(opening: TokenType) -> Option<TokenType> {
    match opening {
        TokenType::Terminal(TerminalToken::OpenBrace) => {
            Some(TokenType::Terminal(TerminalToken::CloseBrace))
        }
        TokenType::Terminal(TerminalToken::OpenSquare) => {
            Some(TokenType::Terminal(TerminalToken::CloseSquare))
        }
        TokenType::Terminal(TerminalToken::OpenBracket) => {
            Some(TokenType::Terminal(TerminalToken::CloseBracket))
        }
        TokenType::Terminal(TerminalToken::OpenAttributes) => {
            Some(TokenType::Terminal(TerminalToken::CloseAttributes))
        }
        _ => None,
    }
}

struct Layer<'s> {
    open_index: usize,
    close_ty: TokenType<'s>,
}

pub fn tokenize(val: &str, flavor: Flavor) -> Result<Vec<TokenItem>, LexerError> {
    let mut items = Vec::<TokenItem>::new();
    let mut layers = VecDeque::<Layer>::new();

    for maybe_token in TokenIter::new(val, flavor) {
        let token = maybe_token?;
        let token_index = items.len();

        // If this token matches the top layer's close token, pop the layer.
        if let Some(top_layer) = layers.back() {
            if top_layer.close_ty == token.ty {
                items[top_layer.open_index].close_index = Some(token_index);
                layers.pop_back();
            }
        }

        // If this token is a valid opening token, push a new layer.
        if let Some(close_ty) = closing_token(token.ty) {
            layers.push_back(Layer {
                open_index: token_index,
                close_ty,
            });
        }

        items.push(TokenItem {
            token,
            close_index: None,
        });
    }

    // If there are remaining layers, there are one or more unmatched opening tokens. Otherwise
    // at this point tokenization is successful.
    match layers.back() {
        None => Ok(items),
        Some(layer) => {
            let open_token = &items[layer.open_index].token;
            Err(LexerError::new(
                open_token.range.clone(),
                LexerErrorType::UnmatchedOpener {
                    open: open_token.ty,
                    close: layer.close_ty,
                },
            ))
        }
    }
}
