use crate::lexer::comment::try_comment;
use crate::lexer::error::{LexerError, LexerErrorType};
use crate::lexer::identifier::try_identifier;
use crate::lexer::literal::try_literal;
use crate::lexer::parse_str::ParseStr;
use crate::lexer::symbol::try_symbol;
use crate::token::{Comment, Token, TokenLine, TokenType};
use crate::Flavor;

macro_rules! try_some {
    ($val:expr) => {
        match $val {
            Ok(val) => val,
            Err(err) => return Some(Err(err)),
        }
    };
}

pub struct TokenIter<'s> {
    val: ParseStr<'s>,
    flavor: Flavor,
    current_token: Option<Token<'s>>,
    before_lines: Vec<TokenLine<'s>>,
    current_line_comments: Vec<Comment<'s>>,
}

impl<'s> TokenIter<'s> {
    pub fn new(val: &'s str, flavor: Flavor) -> TokenIter<'s> {
        TokenIter {
            val: ParseStr::new(val),
            flavor,
            current_token: None,
            before_lines: Vec::new(),
            current_line_comments: Vec::new(),
        }
    }
}

impl<'s> Iterator for TokenIter<'s> {
    type Item = Result<Token<'s>, LexerError<'s>>;

    fn next(&mut self) -> Option<Self::Item> {
        // Scan through any newlines and comments.
        while !self.val.is_ended() {
            // Note that trim_start does not trim newlines.
            self.val = self.val.trim_start();

            // Input could now have ended if it only contained whitespace.
            if self.val.is_ended() {
                break;
            }

            if let Some(remaining) = self.val.strip_prefix("\n") {
                self.val = remaining;

                let line = TokenLine {
                    comments: std::mem::take(&mut self.current_line_comments),
                };

                // If there is an existing token, a newline indicates no more comment information
                // can be added to it, so it can be yielded.
                if let Some(mut current_token) = self.current_token.take() {
                    debug_assert!(current_token.new_line.is_none());
                    current_token.new_line = Some(line);
                    return Some(Ok(current_token));
                }

                // Add the previous line of comments to the next tokens `before_lines` list.
                self.before_lines.push(line);
            } else if let Some((comment, remaining)) = try_some!(try_comment(self.val)) {
                self.val = remaining;
                self.current_line_comments.push(comment);
            } else if let Some((token_ty, remaining)) =
                try_some!(try_token_ty(self.val, self.flavor))
            {
                let token = Token {
                    ty: token_ty,
                    range: self.val.start_offset()..remaining.start_offset(),

                    comments: std::mem::take(&mut self.current_line_comments),
                    before_lines: std::mem::take(&mut self.before_lines),

                    // May be filled later if a \n is encountered.
                    new_line: None,
                };
                self.val = remaining;

                // Store the new token, so continuing comments can be attached to it before either
                // a newline or another token is encountered.
                // If there is an existing token stored, that existing token can now be yielded.
                let existing_token = self.current_token.replace(token);

                if let Some(token) = existing_token {
                    return Some(Ok(token));
                }
            } else {
                // Not a newline, not a comment, not a token.
                return Some(Err(LexerError::new(
                    LexerErrorType::InvalidInput,
                    self.val.start_offset()..self.val.start_offset(),
                )));
            }
        }

        // End of input.
        if let Some(token) = self.current_token.take() {
            return Some(Ok(token));
        }
        if !self.current_line_comments.is_empty() || !self.before_lines.is_empty() {
            return Some(Ok(Token {
                ty: TokenType::Empty,
                range: self.val.end_offset()..self.val.end_offset(),
                comments: std::mem::take(&mut self.current_line_comments),
                before_lines: std::mem::take(&mut self.before_lines),
                new_line: None,
            }));
        }

        None
    }
}

fn try_token_ty(
    val: ParseStr,
    flavor: Flavor,
) -> Result<Option<(TokenType, ParseStr)>, LexerError> {
    if let Some((literal, remaining)) = try_literal(val)? {
        return Ok(Some((TokenType::Literal(literal), remaining)));
    }
    if let Some((symbol, remaining)) = try_symbol(val, flavor) {
        return Ok(Some((TokenType::Terminal(symbol), remaining)));
    }
    if let Some((identifier, remaining)) = try_identifier(val, flavor) {
        return Ok(Some((identifier, remaining)));
    }

    Ok(None)
}
