mod array;
mod class;
mod combinator;
mod context;
mod control;
mod enum_;
mod error;
mod expression;
mod function;
mod global;
mod identifier;
mod list;
mod operator;
mod statement;
mod struct_;
mod table;
mod token;
mod token_list;
mod type_;
mod variable;

pub use self::context::ContextType;
pub use self::error::{ParseError, ParseErrorContext, ParseErrorType};
use crate::ast::Program;

use crate::lexer::TokenItem;
use crate::parser::statement::statement;
use crate::parser::token_list::TokenList;

type ParseResult<'s, T> = Result<(TokenList<'s>, T), ParseError>;

pub fn parse<'s>(items: &'s [TokenItem<'s>]) -> Result<Program<'s>, ParseError> {
    let mut tokens = TokenList::new(items);
    let mut statements = Vec::new();
    while !tokens.is_ended() {
        let (new_tokens, statement) = statement(tokens)?;
        tokens = new_tokens;
        statements.push(statement);
    }
    Ok(Program { statements })
}
