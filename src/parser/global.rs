use crate::ast::GlobalDeclaration;
use crate::parser::combinator::{alternative, first_of, map};
use crate::parser::identifier::identifier;
use crate::parser::list::separated_list1;
use crate::parser::statement::{
    class_declaration_statement, const_statement, enum_statement, struct_declaration_statement,
    typedef_statement, var_declaration_statement,
};
use crate::parser::token::terminal;
use crate::parser::variable::var_initializer;
use crate::parser::{ContextType, ParseError, ParseErrorType, ParseResult, TokenList};
use crate::token::TerminalToken;

pub fn global_declaration(tokens: TokenList) -> ParseResult<GlobalDeclaration> {
    first_of(
        tokens,
        [
            function_global,
            untyped_var_global,
            const_global,
            enum_global,
            class_declaration_global,
            struct_declaration_global,
            type_global,
            typed_var_global,
        ],
        |_| {
            Err(ParseError::new(
                ParseErrorType::ExpectedGlobalDeclaration,
                tokens.start_index(),
            ))
        },
    )
}

fn function_global(tokens: TokenList) -> ParseResult<GlobalDeclaration> {
    alternative(
        tokens,
        ContextType::FunctionGlobal,
        |tokens| terminal(tokens, TerminalToken::Function),
        |tokens, function| {
            let (tokens, name) = separated_list1(tokens, identifier, |tokens| {
                terminal(tokens, TerminalToken::Namespace)
            })?;
            Ok((tokens, GlobalDeclaration::Function { function, name }))
        },
    )
}

fn untyped_var_global(tokens: TokenList) -> ParseResult<GlobalDeclaration> {
    let (tokens, name) = identifier(tokens)?;
    let (tokens, initializer) = var_initializer(tokens)?;

    Ok((tokens, GlobalDeclaration::UntypedVar { name, initializer }))
}

fn const_global(tokens: TokenList) -> ParseResult<GlobalDeclaration> {
    map(const_statement(tokens), GlobalDeclaration::Const)
}

fn enum_global(tokens: TokenList) -> ParseResult<GlobalDeclaration> {
    map(enum_statement(tokens), GlobalDeclaration::Enum)
}

fn class_declaration_global(tokens: TokenList) -> ParseResult<GlobalDeclaration> {
    map(
        class_declaration_statement(tokens),
        GlobalDeclaration::Class,
    )
}

fn struct_declaration_global(tokens: TokenList) -> ParseResult<GlobalDeclaration> {
    map(
        struct_declaration_statement(tokens),
        GlobalDeclaration::Struct,
    )
}

fn type_global(tokens: TokenList) -> ParseResult<GlobalDeclaration> {
    map(typedef_statement(tokens), GlobalDeclaration::Type)
}

fn typed_var_global(tokens: TokenList) -> ParseResult<GlobalDeclaration> {
    map(
        var_declaration_statement(tokens),
        GlobalDeclaration::TypedVar,
    )
}
