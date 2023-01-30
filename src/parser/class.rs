use crate::ast::{ClassDeclaration, ClassExtends, ClassMember, ClassMemberType, Precedence};
use crate::parser::combinator::{alternative, first_of, opt, span};
use crate::parser::expression::{expression, table_delimited};
use crate::parser::function::function_declaration;
use crate::parser::identifier::{identifier, method_identifier};
use crate::parser::list::many;
use crate::parser::token::terminal;
use crate::parser::type_::type_;
use crate::parser::variable::var_initializer;
use crate::parser::{ContextType, ParseError, ParseErrorType, ParseResult, TokenList};
use crate::token::TerminalToken;

pub fn class_declaration(tokens: TokenList) -> ParseResult<ClassDeclaration> {
    let (tokens, extends) = opt(tokens, class_extends(tokens))?;
    span(
        tokens,
        ContextType::ClassDeclaration,
        TerminalToken::OpenBrace,
        TerminalToken::CloseBrace,
        |tokens, open, close| {
            let (tokens, members) = many(tokens, class_member)?;
            Ok((
                tokens,
                ClassDeclaration {
                    extends,
                    open,
                    members,
                    close,
                },
            ))
        },
    )
}

pub fn class_extends(tokens: TokenList) -> ParseResult<ClassExtends> {
    alternative(
        tokens,
        ContextType::ClassExtends,
        |tokens| terminal(tokens, TerminalToken::Extends),
        |tokens, extends| {
            let (tokens, name) = expression(tokens, Precedence::None)?;
            Ok((
                tokens,
                ClassExtends {
                    extends,
                    name: Box::new(name),
                },
            ))
        },
    )
}

pub fn class_member(tokens: TokenList) -> ParseResult<ClassMember> {
    let (tokens, attributes) = opt(
        tokens,
        table_delimited(
            tokens,
            TerminalToken::OpenAttributes,
            TerminalToken::CloseAttributes,
        ),
    )?;
    let (tokens, ty) = class_member_type(tokens)?;
    Ok((tokens, ClassMember { attributes, ty }))
}

fn class_member_type(tokens: TokenList) -> ParseResult<ClassMemberType> {
    first_of(
        tokens,
        [
            // Must be before other types to ensure the return type is parsed.
            function_class_member,
            property_class_member,
            computed_property_class_member,
            constructor_class_member,
        ],
        |_| {
            Err(ParseError::new(
                ParseErrorType::ExpectedClassMember,
                tokens.start_index(),
            ))
        },
    )
}

fn property_class_member(tokens: TokenList) -> ParseResult<ClassMemberType> {
    alternative(
        tokens,
        ContextType::PropertyClassMember,
        |tokens| {
            let (tokens, static_) = opt(tokens, terminal(tokens, TerminalToken::Static))?;
            let (tokens, name) = identifier(tokens)?;
            Ok((tokens, (static_, name)))
        },
        |tokens, (static_, name)| {
            let (tokens, initializer) = var_initializer(tokens)?;
            let (tokens, semicolon) = opt(tokens, terminal(tokens, TerminalToken::Semicolon))?;
            Ok((
                tokens,
                ClassMemberType::Property {
                    static_,
                    name,
                    initializer,
                    semicolon,
                },
            ))
        },
    )
}

fn computed_property_class_member(tokens: TokenList) -> ParseResult<ClassMemberType> {
    alternative(
        tokens,
        ContextType::ComputedPropertyClassMember,
        |tokens| {
            let (tokens, static_) = opt(tokens, terminal(tokens, TerminalToken::Static))?;
            span(
                tokens,
                ContextType::ComputedPropertyClassMember,
                TerminalToken::OpenSquare,
                TerminalToken::CloseSquare,
                |tokens, open, close| {
                    let (tokens, name) = expression(tokens, Precedence::None)?;
                    Ok((tokens, (open, static_, name, close)))
                },
            )
        },
        |tokens, (open, static_, name, close)| {
            let (tokens, initializer) = var_initializer(tokens)?;
            let (tokens, semicolon) = opt(tokens, terminal(tokens, TerminalToken::Semicolon))?;
            Ok((
                tokens,
                ClassMemberType::ComputedProperty {
                    static_,
                    open,
                    name: Box::new(name),
                    close,
                    initializer,
                    semicolon,
                },
            ))
        },
    )
}

fn constructor_class_member(tokens: TokenList) -> ParseResult<ClassMemberType> {
    alternative(
        tokens,
        ContextType::ConstructorClassMember,
        |tokens| terminal(tokens, TerminalToken::Constructor),
        |tokens, constructor| {
            let (tokens, declaration) = function_declaration(tokens)?;
            Ok((
                tokens,
                ClassMemberType::Constructor {
                    constructor,
                    declaration: Box::new(declaration),
                },
            ))
        },
    )
}

fn function_class_member(tokens: TokenList) -> ParseResult<ClassMemberType> {
    alternative(
        tokens,
        ContextType::FunctionClassMember,
        |tokens| {
            let (tokens, return_type) = opt(tokens, type_(tokens))?;
            let (tokens, function) = terminal(tokens, TerminalToken::Function)?;
            Ok((tokens, (return_type, function)))
        },
        |tokens, (return_type, function)| {
            let (tokens, name) = method_identifier(tokens)?;
            let (tokens, declaration) = function_declaration(tokens)?;
            Ok((
                tokens,
                ClassMemberType::Function {
                    return_type,
                    function,
                    name,
                    declaration: Box::new(declaration),
                },
            ))
        },
    )
}
