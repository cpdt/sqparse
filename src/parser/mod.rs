mod array;
mod class;
mod context;
mod control;
mod enum_;
mod error;
mod expression;
mod function;
mod global;
mod identifier;
mod operator;
mod parse_result_ext;
mod slot;
mod statement;
mod struct_;
mod table;
mod token_list;
mod token_list_ext;
mod type_;
mod variable;

pub use self::context::ContextType;
pub use self::error::{ParseError, ParseErrorContext, ParseErrorType};
use crate::ast::Program;

use crate::lexer::TokenItem;
use crate::parser::statement::statement;
use crate::parser::token_list::TokenList;
use crate::parser::token_list_ext::TokenListExt;

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
    let tokens = TokenList::new(items);
    let (tokens, statements) = tokens.many_until_ended(statement)?;
    assert!(tokens.is_ended());
    Ok(Program { statements })
}
