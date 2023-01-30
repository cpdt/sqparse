use crate::ast::{
    BlockStatement, BreakStatement, ClassDeclarationStatement, ConstStatement,
    ConstructorDeclarationStatement, ContinueStatement, DelayThreadStatement, DoWhileStatement,
    EmptyStatement, EnumStatement, ExpressionStatement, ForStatement, ForeachStatement,
    FunctionDeclarationStatement, GlobalStatement, GlobalizeAllFunctionsStatement, IfStatement,
    Precedence, ReturnStatement, SeparatedListTrailing1, Statement, StatementType,
    StructDeclarationStatement, SwitchStatement, ThreadStatement, ThrowStatement,
    TryCatchStatement, TypedefStatement, UntypedStatement, VarDeclarationStatement, WaitStatement,
    WaitThreadStatement, WhileStatement, YieldStatement,
};
use crate::parser::class::class_declaration;
use crate::parser::combinator::{alt, alternative, first_of, map, opt, prevent_ending_line, span};
use crate::parser::control::{for_declaration, foreach_index, foreach_value, if_else, switch_case};
use crate::parser::enum_::enum_entry;
use crate::parser::error::InternalErrorType;
use crate::parser::expression::expression;
use crate::parser::function::function_declaration;
use crate::parser::global::global_declaration;
use crate::parser::identifier::identifier;
use crate::parser::list::{many, separated_list1, separated_list_trailing0};
use crate::parser::struct_::struct_declaration;
use crate::parser::token::terminal;
use crate::parser::type_::type_;
use crate::parser::variable::{var_declaration, var_initializer};
use crate::parser::{ContextType, ParseError, ParseErrorType, ParseResult, TokenList};
use crate::token::{TerminalToken, Token, TokenType};

pub fn statement(tokens: TokenList) -> ParseResult<Statement> {
    // Empty statement?
    if let Ok((tokens, end)) = terminal(tokens, TerminalToken::Semicolon) {
        return Ok((
            tokens,
            Statement {
                ty: StatementType::Empty(EmptyStatement { empty: None }),
                end: Some(end),
            },
        ));
    }

    let (tokens, ty) = statement_type(tokens)?;
    let (tokens, end) = opt(tokens, terminal(tokens, TerminalToken::Semicolon))?;
    Ok((tokens, Statement { ty, end }))
}

pub fn statement_type(tokens: TokenList) -> ParseResult<StatementType> {
    first_of(
        tokens,
        [
            |tokens| map(empty_statement(tokens), StatementType::Empty),
            // Must be before `function_declaration_statement` to ensure it doesn't match the
            // function-like syntax.
            |tokens| {
                map(
                    constructor_declaration_statement(tokens),
                    StatementType::ConstructorDeclaration,
                )
            },
            // Must be before other types to ensure the return type/var type is parsed.
            |tokens| {
                map(
                    function_declaration_statement(tokens),
                    StatementType::FunctionDeclaration,
                )
            },
            |tokens| {
                map(
                    var_declaration_statement(tokens),
                    StatementType::VarDeclaration,
                )
            },
            |tokens| map(block_statement(tokens), StatementType::Block),
            |tokens| map(if_statement(tokens), StatementType::If),
            |tokens| map(while_statement(tokens), StatementType::While),
            |tokens| map(do_while_statement(tokens), StatementType::DoWhile),
            |tokens| map(switch_statement(tokens), StatementType::Switch),
            |tokens| map(for_statement(tokens), StatementType::For),
            |tokens| map(foreach_statement(tokens), StatementType::ForeachStatement),
            |tokens| map(break_statement(tokens), StatementType::Break),
            |tokens| map(continue_statement(tokens), StatementType::Continue),
            |tokens| map(return_statement(tokens), StatementType::Return),
            |tokens| map(yield_statement(tokens), StatementType::Yield),
            |tokens| {
                map(
                    class_declaration_statement(tokens),
                    StatementType::ClassDeclaration,
                )
            },
            |tokens| map(try_catch_statement(tokens), StatementType::TryCatch),
            |tokens| map(throw_statement(tokens), StatementType::Throw),
            |tokens| map(const_statement(tokens), StatementType::Const),
            |tokens| map(enum_statement(tokens), StatementType::Enum),
            |tokens| map(thread_statement(tokens), StatementType::Thread),
            |tokens| map(delay_thread_statement(tokens), StatementType::DelayThread),
            |tokens| map(wait_thread_statement(tokens), StatementType::WaitThread),
            |tokens| map(wait_statement(tokens), StatementType::Wait),
            |tokens| {
                map(
                    struct_declaration_statement(tokens),
                    StatementType::StructDeclaration,
                )
            },
            |tokens| map(typedef_statement(tokens), StatementType::Typedef),
            |tokens| map(global_statement(tokens), StatementType::Global),
            |tokens| {
                map(
                    globalize_all_functions_statement(tokens),
                    StatementType::GlobalizeAllFunctions,
                )
            },
            |tokens| map(untyped_statement(tokens), StatementType::Untyped),
            // This is last because it is potentially expensive.
            |tokens| map(expression_statement(tokens), StatementType::Expression),
        ],
        |_| {
            Err(ParseError::new(
                ParseErrorType::ExpectedStatement,
                tokens.start_index(),
            ))
        },
    )
}

pub fn empty_statement(tokens: TokenList) -> ParseResult<EmptyStatement> {
    if let Some((tokens, item)) = tokens.split_first() {
        if item.token.ty == TokenType::Empty {
            return Ok((
                tokens,
                EmptyStatement {
                    empty: Some(&item.token),
                },
            ));
        }
    }

    Err(ParseError::new(
        ParseErrorType::Internal(InternalErrorType::Empty),
        tokens.start_index(),
    ))
}

pub fn block_statement(tokens: TokenList) -> ParseResult<BlockStatement> {
    span(
        tokens,
        ContextType::BlockStatement,
        TerminalToken::OpenBrace,
        TerminalToken::CloseBrace,
        |tokens, open, close| {
            let (tokens, statements) = many(tokens, statement)?;
            Ok((
                tokens,
                BlockStatement {
                    open,
                    statements,
                    close,
                },
            ))
        },
    )
}

pub fn if_statement(tokens: TokenList) -> ParseResult<IfStatement> {
    alternative(
        tokens,
        ContextType::IfStatement,
        |tokens| terminal(tokens, TerminalToken::If),
        |tokens, if_| {
            let (tokens, (open, condition, close)) = span(
                tokens,
                ContextType::IfStatement,
                TerminalToken::OpenBracket,
                TerminalToken::CloseBracket,
                |tokens, open, close| {
                    let (tokens, condition) = expression(tokens, Precedence::None)?;
                    Ok((tokens, (open, condition, close)))
                },
            )?;
            let (tokens, body) = statement(tokens)?;
            let (tokens, else_) = opt(tokens, if_else(tokens))?;

            Ok((
                tokens,
                IfStatement {
                    if_,
                    open,
                    condition: Box::new(condition),
                    close,
                    body: Box::new(body),
                    else_,
                },
            ))
        },
    )
}

pub fn while_statement(tokens: TokenList) -> ParseResult<WhileStatement> {
    alternative(
        tokens,
        ContextType::WhileStatement,
        |tokens| terminal(tokens, TerminalToken::While),
        |tokens, while_| {
            let (tokens, (open, condition, close)) = span(
                tokens,
                ContextType::WhileStatement,
                TerminalToken::OpenBracket,
                TerminalToken::CloseBracket,
                |tokens, open, close| {
                    let (tokens, condition) = expression(tokens, Precedence::None)?;
                    Ok((tokens, (open, condition, close)))
                },
            )?;
            let (tokens, body) = statement(tokens)?;

            Ok((
                tokens,
                WhileStatement {
                    while_,
                    open,
                    condition: Box::new(condition),
                    close,
                    body: Box::new(body),
                },
            ))
        },
    )
}

pub fn do_while_statement(tokens: TokenList) -> ParseResult<DoWhileStatement> {
    alternative(
        tokens,
        ContextType::DoWhileStatement,
        |tokens| terminal(tokens, TerminalToken::Do),
        |tokens, do_| {
            let (tokens, body) = statement(tokens)?;
            let (tokens, while_) = terminal(tokens, TerminalToken::While)?;

            span(
                tokens,
                ContextType::DoWhileStatement,
                TerminalToken::OpenBracket,
                TerminalToken::CloseBracket,
                |tokens, open, close| {
                    let (tokens, condition) = expression(tokens, Precedence::None)?;
                    Ok((
                        tokens,
                        DoWhileStatement {
                            do_,
                            body: Box::new(body),
                            while_,
                            open,
                            condition: Box::new(condition),
                            close,
                        },
                    ))
                },
            )
        },
    )
}

pub fn switch_statement(tokens: TokenList) -> ParseResult<SwitchStatement> {
    alternative(
        tokens,
        ContextType::SwitchStatement,
        |tokens| terminal(tokens, TerminalToken::Switch),
        |tokens, switch| {
            let (tokens, (open_condition, condition, close_condition)) = span(
                tokens,
                ContextType::SwitchStatement,
                TerminalToken::OpenBracket,
                TerminalToken::CloseBracket,
                |tokens, open, close| {
                    let (tokens, condition) = expression(tokens, Precedence::None)?;
                    Ok((tokens, (open, condition, close)))
                },
            )?;

            span(
                tokens,
                ContextType::SwitchStatement,
                TerminalToken::OpenBrace,
                TerminalToken::CloseBrace,
                |tokens, open_cases, close_cases| {
                    let (tokens, cases) = many(tokens, switch_case)?;
                    Ok((
                        tokens,
                        SwitchStatement {
                            switch,
                            open_condition,
                            condition: Box::new(condition),
                            close_condition,
                            open_cases,
                            cases,
                            close_cases,
                        },
                    ))
                },
            )
        },
    )
}

pub fn for_statement(tokens: TokenList) -> ParseResult<ForStatement> {
    alternative(
        tokens,
        ContextType::ForStatement,
        |tokens| terminal(tokens, TerminalToken::For),
        |tokens, for_| {
            let (
                tokens,
                (open, initializer, semicolon_1, condition, semicolon_2, increment, close),
            ) = span(
                tokens,
                ContextType::ForStatement,
                TerminalToken::OpenBracket,
                TerminalToken::CloseBracket,
                |tokens, open, close| {
                    let (tokens, initializer) = opt(tokens, for_declaration(tokens))?;
                    let (tokens, semicolon_1) = terminal(tokens, TerminalToken::Semicolon)?;
                    let (tokens, condition) = opt(tokens, expression(tokens, Precedence::None))?;
                    let (tokens, semicolon_2) = terminal(tokens, TerminalToken::Semicolon)?;
                    let (tokens, increment) = opt(tokens, expression(tokens, Precedence::None))?;
                    Ok((
                        tokens,
                        (
                            open,
                            initializer,
                            semicolon_1,
                            condition,
                            semicolon_2,
                            increment,
                            close,
                        ),
                    ))
                },
            )?;
            let (tokens, body) = statement(tokens)?;
            Ok((
                tokens,
                ForStatement {
                    for_,
                    open,
                    initializer,
                    semicolon_1,
                    condition: condition.map(Box::new),
                    semicolon_2,
                    increment: increment.map(Box::new),
                    close,
                    body: Box::new(body),
                },
            ))
        },
    )
}

pub fn foreach_statement(tokens: TokenList) -> ParseResult<ForeachStatement> {
    alternative(
        tokens,
        ContextType::ForeachStatement,
        |tokens| terminal(tokens, TerminalToken::Foreach),
        |tokens, foreach| {
            let (tokens, (open, index, value_type, value_name, in_, array, close)) = span(
                tokens,
                ContextType::ForeachStatement,
                TerminalToken::OpenBracket,
                TerminalToken::CloseBracket,
                |tokens, open, close| {
                    let (tokens, index) = opt(tokens, foreach_index(tokens))?;
                    let (tokens, (value_type, value_name, in_)) = foreach_value(tokens)?;
                    let (tokens, array) = expression(tokens, Precedence::None)?;
                    Ok((
                        tokens,
                        (open, index, value_type, value_name, in_, array, close),
                    ))
                },
            )?;
            let (tokens, body) = statement(tokens)?;
            Ok((
                tokens,
                ForeachStatement {
                    foreach,
                    open,
                    index,
                    value_type,
                    value_name,
                    in_,
                    array: Box::new(array),
                    close,
                    body: Box::new(body),
                },
            ))
        },
    )
}

pub fn break_statement(tokens: TokenList) -> ParseResult<BreakStatement> {
    let (tokens, break_) = terminal(tokens, TerminalToken::Break)?;
    Ok((tokens, BreakStatement { break_ }))
}

pub fn continue_statement(tokens: TokenList) -> ParseResult<ContinueStatement> {
    let (tokens, continue_) = terminal(tokens, TerminalToken::Continue)?;
    Ok((tokens, ContinueStatement { continue_ }))
}

pub fn return_statement(tokens: TokenList) -> ParseResult<ReturnStatement> {
    alternative(
        tokens,
        ContextType::ReturnStatement,
        |tokens| terminal(tokens, TerminalToken::Return),
        |tokens, return_| {
            if tokens.is_newline() {
                Ok((
                    tokens,
                    ReturnStatement {
                        return_,
                        value: None,
                    },
                ))
            } else {
                let (tokens, value) = opt(tokens, expression(tokens, Precedence::None))?;
                Ok((
                    tokens,
                    ReturnStatement {
                        return_,
                        value: value.map(Box::new),
                    },
                ))
            }
        },
    )
}

pub fn yield_statement(tokens: TokenList) -> ParseResult<YieldStatement> {
    alternative(
        tokens,
        ContextType::YieldStatement,
        |tokens| terminal(tokens, TerminalToken::Yield),
        |tokens, yield_| {
            if tokens.is_newline() {
                Ok((
                    tokens,
                    YieldStatement {
                        yield_,
                        value: None,
                    },
                ))
            } else {
                let (tokens, value) = opt(tokens, expression(tokens, Precedence::None))?;
                Ok((
                    tokens,
                    YieldStatement {
                        yield_,
                        value: value.map(Box::new),
                    },
                ))
            }
        },
    )
}

pub fn class_declaration_statement(tokens: TokenList) -> ParseResult<ClassDeclarationStatement> {
    alternative(
        tokens,
        ContextType::ClassStatement,
        |tokens| terminal(tokens, TerminalToken::Class),
        |tokens, class| {
            let (tokens, name) = expression(tokens, Precedence::None)?;
            let (tokens, declaration) = class_declaration(tokens)?;
            Ok((
                tokens,
                ClassDeclarationStatement {
                    class,
                    name: Box::new(name),
                    declaration,
                },
            ))
        },
    )
}

pub fn try_catch_statement(tokens: TokenList) -> ParseResult<TryCatchStatement> {
    alternative(
        tokens,
        ContextType::TryCatchStatement,
        |tokens| terminal(tokens, TerminalToken::Try),
        |tokens, try_| {
            let (tokens, body) = statement(tokens)?;
            let (tokens, catch) = terminal(tokens, TerminalToken::Catch)?;
            let (tokens, (open, catch_name, close)) = span(
                tokens,
                ContextType::TryCatchStatement,
                TerminalToken::OpenBracket,
                TerminalToken::CloseBracket,
                |tokens, open, close| {
                    let (tokens, catch_name) = identifier(tokens)?;
                    Ok((tokens, (open, catch_name, close)))
                },
            )?;
            let (tokens, catch_body) = statement(tokens)?;
            Ok((
                tokens,
                TryCatchStatement {
                    try_,
                    body: Box::new(body),
                    catch,
                    open,
                    catch_name,
                    close,
                    catch_body: Box::new(catch_body),
                },
            ))
        },
    )
}

pub fn throw_statement(tokens: TokenList) -> ParseResult<ThrowStatement> {
    alternative(
        tokens,
        ContextType::ThrowStatement,
        |tokens| terminal(tokens, TerminalToken::Throw),
        |tokens, throw| {
            let (tokens, value) = expression(tokens, Precedence::None)?;
            Ok((
                tokens,
                ThrowStatement {
                    throw,
                    value: Box::new(value),
                },
            ))
        },
    )
}

pub fn const_statement(tokens: TokenList) -> ParseResult<ConstStatement> {
    alternative(
        tokens,
        ContextType::ConstStatement,
        |tokens| terminal(tokens, TerminalToken::Const),
        |tokens, const_| {
            alt(typed_const_statement(tokens, const_))
                .unwrap_or_else(|| untyped_const_statement(tokens, const_))
        },
    )
}

fn typed_const_statement<'s>(
    tokens: TokenList<'s>,
    const_: &'s Token<'s>,
) -> ParseResult<'s, ConstStatement<'s>> {
    let (tokens, const_type) = prevent_ending_line(tokens, type_(tokens))?;
    let (tokens, name) = identifier(tokens)?;
    let (tokens, initializer) = var_initializer(tokens)?;
    Ok((
        tokens,
        ConstStatement {
            const_,
            const_type: Some(const_type),
            name,
            initializer,
        },
    ))
}

fn untyped_const_statement<'s>(
    tokens: TokenList<'s>,
    const_: &'s Token<'s>,
) -> ParseResult<'s, ConstStatement<'s>> {
    let (tokens, name) = identifier(tokens)?;
    let (tokens, initializer) = var_initializer(tokens)?;
    Ok((
        tokens,
        ConstStatement {
            const_,
            const_type: None,
            name,
            initializer,
        },
    ))
}

pub fn enum_statement(tokens: TokenList) -> ParseResult<EnumStatement> {
    alternative(
        tokens,
        ContextType::EnumStatement,
        |tokens| terminal(tokens, TerminalToken::Enum),
        |tokens, enum_| {
            let (tokens, name) = identifier(tokens)?;
            span(
                tokens,
                ContextType::EnumStatement,
                TerminalToken::OpenBrace,
                TerminalToken::CloseBrace,
                |tokens, open, close| {
                    let (tokens, entries) = many(tokens, enum_entry)?;
                    Ok((
                        tokens,
                        EnumStatement {
                            enum_,
                            name,
                            open,
                            entries,
                            close,
                        },
                    ))
                },
            )
        },
    )
}

pub fn expression_statement(tokens: TokenList) -> ParseResult<ExpressionStatement> {
    let (tokens, value) = expression(tokens, Precedence::None)?;
    Ok((
        tokens,
        ExpressionStatement {
            value: Box::new(value),
        },
    ))
}

pub fn constructor_declaration_statement(
    tokens: TokenList,
) -> ParseResult<ConstructorDeclarationStatement> {
    alternative(
        tokens,
        ContextType::FunctionDeclarationStatement,
        |tokens| {
            let (tokens, function) = terminal(tokens, TerminalToken::Function)?;
            let (tokens, mut last_name) = identifier(tokens)?;
            let (mut tokens, mut last_namespace) = terminal(tokens, TerminalToken::Namespace)?;
            let mut namespaces = Vec::new();

            while let (next_tokens, Some(next_name)) = opt(tokens, identifier(tokens))? {
                let (next_tokens, next_namespace) =
                    terminal(next_tokens, TerminalToken::Namespace)?;
                namespaces.push((last_name, last_namespace));
                last_name = next_name;
                last_namespace = next_namespace;
                tokens = next_tokens;
            }

            let (tokens, constructor) = terminal(tokens, TerminalToken::Constructor)?;
            Ok((
                tokens,
                (function, namespaces, last_name, last_namespace, constructor),
            ))
        },
        |tokens, (function, namespaces, last_name, last_namespace, constructor)| {
            let (tokens, declaration) = function_declaration(tokens)?;
            Ok((
                tokens,
                ConstructorDeclarationStatement {
                    function,
                    namespaces,
                    last_name,
                    last_namespace,
                    constructor,
                    declaration,
                },
            ))
        },
    )
}

pub fn function_declaration_statement(
    tokens: TokenList,
) -> ParseResult<FunctionDeclarationStatement> {
    alternative(
        tokens,
        ContextType::FunctionDeclarationStatement,
        |tokens| {
            let (tokens, return_type) = prevent_ending_line(
                tokens,
                opt(tokens, type_(tokens).map_err(|err| err.into_non_fatal())),
            )?;
            let (tokens, function) = terminal(tokens, TerminalToken::Function)?;
            Ok((tokens, (return_type, function)))
        },
        |tokens, (return_type, function)| {
            let (tokens, name) = separated_list1(tokens, identifier, |tokens| {
                terminal(tokens, TerminalToken::Namespace)
            })?;
            let (tokens, declaration) = function_declaration(tokens)?;
            Ok((
                tokens,
                FunctionDeclarationStatement {
                    return_type,
                    function,
                    name,
                    declaration,
                },
            ))
        },
    )
}

pub fn var_declaration_statement(tokens: TokenList) -> ParseResult<VarDeclarationStatement> {
    // Variable declaration statements are very vague syntactically. For better error reporting, we
    // say a token stream is definitely a variable declaration statement if it has a type followed
    // by an identifier. However once that's known we must parse the rest of the declarations,
    // which involves some awkward shuffling.
    alternative(
        tokens,
        ContextType::VarDeclarationStatement,
        |tokens| {
            let (tokens, ty) = type_(tokens).map_err(|err| err.into_non_fatal())?;
            let (tokens, first_declaration) = var_declaration(tokens)?;
            Ok((tokens, (ty, first_declaration)))
        },
        |tokens, (ty, first_declaration)| {
            let (tokens, first_comma_maybe) = opt(tokens, terminal(tokens, TerminalToken::Comma))?;
            let (tokens, declarations) = match first_comma_maybe {
                Some(first_comma) => {
                    let (tokens, declarations) =
                        separated_list_trailing0(tokens, var_declaration, |tokens| {
                            terminal(tokens, TerminalToken::Comma)
                        })?;
                    let declarations = match declarations {
                        Some(mut declarations) => {
                            declarations
                                .items
                                .insert(0, (first_declaration, first_comma));
                            declarations
                        }
                        None => SeparatedListTrailing1 {
                            items: Vec::new(),
                            last_item: Box::new(first_declaration),
                            trailing: Some(first_comma),
                        },
                    };
                    (tokens, declarations)
                }
                None => (
                    tokens,
                    SeparatedListTrailing1 {
                        items: Vec::new(),
                        last_item: Box::new(first_declaration),
                        trailing: None,
                    },
                ),
            };
            Ok((tokens, VarDeclarationStatement { ty, declarations }))
        },
    )
}

pub fn thread_statement(tokens: TokenList) -> ParseResult<ThreadStatement> {
    alternative(
        tokens,
        ContextType::ThreadStatement,
        |tokens| terminal(tokens, TerminalToken::Thread),
        |tokens, thread| {
            let (tokens, value) = expression(tokens, Precedence::None)?;
            Ok((
                tokens,
                ThreadStatement {
                    thread,
                    value: Box::new(value),
                },
            ))
        },
    )
}

pub fn delay_thread_statement(tokens: TokenList) -> ParseResult<DelayThreadStatement> {
    alternative(
        tokens,
        ContextType::DelayThreadStatement,
        |tokens| terminal(tokens, TerminalToken::DelayThread),
        |tokens, delay_thread| {
            let (tokens, (open, duration, close)) = span(
                tokens,
                ContextType::DelayThreadStatement,
                TerminalToken::OpenBracket,
                TerminalToken::CloseBracket,
                |tokens, open, close| {
                    let (tokens, duration) = expression(tokens, Precedence::None)?;
                    Ok((tokens, (open, duration, close)))
                },
            )?;
            let (tokens, value) = expression(tokens, Precedence::None)?;
            Ok((
                tokens,
                DelayThreadStatement {
                    delay_thread,
                    open,
                    duration: Box::new(duration),
                    close,
                    value: Box::new(value),
                },
            ))
        },
    )
}

pub fn wait_thread_statement(tokens: TokenList) -> ParseResult<WaitThreadStatement> {
    alternative(
        tokens,
        ContextType::WaitThreadStatement,
        |tokens| terminal(tokens, TerminalToken::WaitThread),
        |tokens, wait_thread| {
            let (tokens, value) = expression(tokens, Precedence::None)?;
            Ok((
                tokens,
                WaitThreadStatement {
                    wait_thread,
                    value: Box::new(value),
                },
            ))
        },
    )
}

pub fn wait_statement(tokens: TokenList) -> ParseResult<WaitStatement> {
    alternative(
        tokens,
        ContextType::WaitStatement,
        |tokens| terminal(tokens, TerminalToken::Wait),
        |tokens, wait| {
            let (tokens, value) = expression(tokens, Precedence::None)?;
            Ok((
                tokens,
                WaitStatement {
                    wait,
                    value: Box::new(value),
                },
            ))
        },
    )
}

pub fn struct_declaration_statement(tokens: TokenList) -> ParseResult<StructDeclarationStatement> {
    alternative(
        tokens,
        ContextType::StructStatement,
        |tokens| terminal(tokens, TerminalToken::Struct),
        |tokens, struct_| {
            let (tokens, name) = identifier(tokens)?;
            let (tokens, declaration) = struct_declaration(tokens)?;
            Ok((
                tokens,
                StructDeclarationStatement {
                    struct_,
                    name,
                    declaration,
                },
            ))
        },
    )
}

pub fn typedef_statement(tokens: TokenList) -> ParseResult<TypedefStatement> {
    alternative(
        tokens,
        ContextType::TypedefStatement,
        |tokens| terminal(tokens, TerminalToken::Typedef),
        |tokens, typedef| {
            let (tokens, name) = identifier(tokens)?;
            let (tokens, ty) = type_(tokens)?;
            Ok((tokens, TypedefStatement { typedef, name, ty }))
        },
    )
}

pub fn global_statement(tokens: TokenList) -> ParseResult<GlobalStatement> {
    alternative(
        tokens,
        ContextType::GlobalStatement,
        |tokens| terminal(tokens, TerminalToken::Global),
        |tokens, global| {
            let (tokens, declaration) = global_declaration(tokens)?;
            Ok((
                tokens,
                GlobalStatement {
                    global,
                    declaration,
                },
            ))
        },
    )
}

pub fn globalize_all_functions_statement(
    tokens: TokenList,
) -> ParseResult<GlobalizeAllFunctionsStatement> {
    let (tokens, globalize_all_functions) = terminal(tokens, TerminalToken::GlobalizeAllFunctions)?;
    Ok((
        tokens,
        GlobalizeAllFunctionsStatement {
            globalize_all_functions,
        },
    ))
}

pub fn untyped_statement(tokens: TokenList) -> ParseResult<UntypedStatement> {
    let (tokens, untyped) = terminal(tokens, TerminalToken::Untyped)?;
    Ok((tokens, UntypedStatement { untyped }))
}
