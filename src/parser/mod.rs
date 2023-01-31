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

/// Parses an input token list into a syntax tree.
///
/// # Example
/// ```
/// use sqparse::{Flavor, parse, tokenize};
///
/// let source = r#"
/// global function MyFunction
///
/// struct {
///     int a
/// } file
///
/// string function MyFunction( List<number> values ) {
///     values.push(1 + 2)
/// }
/// "#;
/// let tokens = tokenize(source, Flavor::SquirrelRespawn).unwrap();
///
/// let program = parse(&tokens).unwrap();
/// assert_eq!(program.statements.len(), 3);
/// ```
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