use crate::ast::GlobalDefinition;
use crate::parser::identifier::identifier;
use crate::parser::parse_result_ext::ParseResultExt;
use crate::parser::statement::{
    class_definition_statement, const_definition_statement, enum_definition_statement,
    struct_definition_statement, type_definition_statement, typed_var_definition_statement,
};
use crate::parser::token_list::TokenList;
use crate::parser::token_list_ext::TokenListExt;
use crate::parser::type_::type_;
use crate::parser::variable::var_initializer;
use crate::parser::ParseResult;
use crate::token::TerminalToken;
use crate::ParseErrorType;

pub fn global_definition(tokens: TokenList) -> ParseResult<GlobalDefinition> {
    function_global(tokens)
        .or_try(|| const_global(tokens))
        .or_try(|| enum_global(tokens))
        .or_try(|| class_global(tokens))
        .or_try(|| struct_global(tokens))
        .or_try(|| type_global(tokens))
        .or_try(|| untyped_var_global(tokens))
        .or_try(|| typed_var_global(tokens))
        .or_error(|| tokens.error(ParseErrorType::ExpectedGlobalDefinition))
}

fn function_global(tokens: TokenList) -> ParseResult<GlobalDefinition> {
    tokens
        .terminal(TerminalToken::Function)
        .determines(|tokens, function| {
            identifier(tokens).map_val(|name| GlobalDefinition::Function { function, name })
        })
}

fn const_global(tokens: TokenList) -> ParseResult<GlobalDefinition> {
    const_definition_statement(tokens).map_val(GlobalDefinition::Const)
}

fn enum_global(tokens: TokenList) -> ParseResult<GlobalDefinition> {
    enum_definition_statement(tokens).map_val(GlobalDefinition::Enum)
}

fn class_global(tokens: TokenList) -> ParseResult<GlobalDefinition> {
    class_definition_statement(tokens).map_val(GlobalDefinition::Class)
}

fn struct_global(tokens: TokenList) -> ParseResult<GlobalDefinition> {
    struct_definition_statement(tokens).map_val(GlobalDefinition::Struct)
}

fn type_global(tokens: TokenList) -> ParseResult<GlobalDefinition> {
    type_definition_statement(tokens).map_val(GlobalDefinition::Type)
}

fn untyped_var_global(tokens: TokenList) -> ParseResult<GlobalDefinition> {
    let (tokens, name) = identifier(tokens)?;
    let (tokens, initializer) = var_initializer(tokens)?;
    Ok((tokens, GlobalDefinition::UntypedVar { name, initializer }))
}

fn typed_var_global(tokens: TokenList) -> ParseResult<GlobalDefinition> {
    let (tokens, type_) = type_(tokens).not_line_ending().not_definite()?;

    typed_var_definition_statement(tokens, type_).map_val(GlobalDefinition::TypedVar)
}
