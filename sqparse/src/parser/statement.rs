use crate::ast::{
    BlockStatement, BreakStatement, ClassDeclarationStatement, ConstStatement, ContinueStatement,
    DelayThreadStatement, DoWhileStatement, EnumEntry, EnumStatement, ExpressionOrDeclaration,
    ForStatement, ForeachIndex, ForeachStatement, FunctionDeclarationStatement, GlobalStatement,
    GlobalType, GlobalizeAllFunctionsStatement, IfElse, IfStatement, ReturnStatement, Statement,
    StatementType, StructDeclarationStatement, SwitchCase, SwitchCaseCondition, SwitchStatement,
    ThreadStatement, ThrowStatement, TryCatchStatement, Type, TypeDeclarationStatement,
    UntypedStatement, VarDeclaration, VarDeclarationStatement, WaitStatement, WaitThreadStatement,
    WhileStatement, YieldStatement,
};
use crate::parser::expression::expression;
use crate::parser::shared::{
    class_declaration, first_matching, function_declaration, identifier, many_separated,
    many_separated_till, not_line_ending, perfect, separated_list1, struct_declaration, terminal,
    var_initializer,
};
use crate::parser::ty::ty;
use crate::parser::TokenList;
use crate::token::{TerminalToken, TokenType};
use crate::IResult;
use nom::branch::alt;
use nom::combinator::{consumed, eof, map, opt, success};
use nom::error::context;
use nom::multi::many_till;
use nom::sequence::{pair, tuple};

pub fn parse_document(i: TokenList) -> IResult<TokenList, Vec<Statement>> {
    many_separated_till(
        statement_type,
        terminal(TerminalToken::Semicolon),
        eof,
        |ty, end_token| Statement { ty, end_token },
    )(i)
    .map(|(i, (statements, _))| (i, statements))
}

fn statement(i: TokenList) -> IResult<TokenList, Statement> {
    let (i, (consumed_input, ty)) = consumed(statement_type)(i)?;

    if i.is_empty() || consumed_input.end_of_line() {
        return Ok((
            i,
            Statement {
                ty,
                end_token: None,
            },
        ));
    }

    let (i, end_token) = opt(terminal(TerminalToken::Semicolon))(i)?;
    Ok((i, Statement { ty, end_token }))
}

fn statement_list(i: TokenList) -> IResult<TokenList, Vec<Statement>> {
    many_separated(
        statement_type,
        terminal(TerminalToken::Semicolon),
        |ty, end_token| Statement { ty, end_token },
    )(i)
}

pub fn inline_statement(i: TokenList) -> IResult<TokenList, Statement> {
    alt((
        map(block_statement, |block_statement| Statement {
            ty: StatementType::Block(block_statement),

            end_token: None,
        }),
        statement,
    ))(i)
}

fn statement_type(i: TokenList) -> IResult<TokenList, StatementType> {
    alt((
        alt((
            map(first_matching(TokenType::Empty), StatementType::Blank),
            map(block_statement, StatementType::Block),
            map(if_statement, StatementType::If),
            map(while_statement, StatementType::While),
            map(do_while_statement, StatementType::DoWhile),
            map(switch_statement, StatementType::Switch),
            map(for_statement, StatementType::For),
            map(foreach_statement, StatementType::Foreach),
            map(break_statement, StatementType::Break),
            map(continue_statement, StatementType::Continue),
            map(return_statement, StatementType::Return),
            map(yield_statement, StatementType::Yield),
            map(class_declaration_statement, StatementType::ClassDeclaration),
            map(try_catch_statement, StatementType::TryCatch),
            map(throw_statement, StatementType::Throw),
            map(const_statement, StatementType::Const),
            map(enum_statement, StatementType::Enum),
            var_or_func_declaration_statement,
        )),
        alt((
            map(thread_statement, StatementType::Thread),
            map(delay_thread_statement, StatementType::DelayThread),
            map(wait_thread_statement, StatementType::WaitThread),
            map(wait_statement, StatementType::Wait),
            map(
                struct_declaration_statement,
                StatementType::StructDeclaration,
            ),
            map(type_declaration_statement, StatementType::TypeDeclaration),
            map(global_statement, StatementType::Global),
            map(
                globalize_all_functions_statement,
                StatementType::GlobalizeAllFunctions,
            ),
            map(untyped_statement, StatementType::Untyped),
        )),
        map(expression, |expr| StatementType::Expression(Box::new(expr))),
        success(StatementType::Empty),
    ))(i)
}

fn block_statement(i: TokenList) -> IResult<TokenList, BlockStatement> {
    map(
        pair(
            terminal(TerminalToken::OpenBrace),
            perfect(
                "block",
                pair(statement_list, terminal(TerminalToken::CloseBrace)),
            ),
        ),
        |(open_token, (body, close_token))| BlockStatement {
            body,

            open_token,
            close_token,
        },
    )(i)
}

fn if_else(i: TokenList) -> IResult<TokenList, IfElse> {
    map(
        pair(
            terminal(TerminalToken::Else),
            perfect("else statement", inline_statement),
        ),
        |(else_token, statement)| IfElse {
            statement: Box::new(statement),

            else_token,
        },
    )(i)
}

fn if_statement(i: TokenList) -> IResult<TokenList, IfStatement> {
    map(
        pair(
            terminal(TerminalToken::If),
            perfect(
                "if statement",
                tuple((
                    terminal(TerminalToken::OpenBracket),
                    expression,
                    terminal(TerminalToken::CloseBracket),
                    inline_statement,
                    opt(if_else),
                )),
            ),
        ),
        |(if_token, (open_condition_token, condition, close_condition_token, body, else_))| {
            IfStatement {
                condition: Box::new(condition),
                body: Box::new(body),
                else_,

                if_token,
                open_condition_token,
                close_condition_token,
            }
        },
    )(i)
}

fn while_statement(i: TokenList) -> IResult<TokenList, WhileStatement> {
    map(
        pair(
            terminal(TerminalToken::While),
            perfect(
                "while loop",
                tuple((
                    terminal(TerminalToken::OpenBracket),
                    expression,
                    terminal(TerminalToken::CloseBracket),
                    inline_statement,
                )),
            ),
        ),
        |(while_token, (open_condition_token, condition, close_condition_token, body))| {
            WhileStatement {
                condition: Box::new(condition),
                body: Box::new(body),

                while_token,
                open_condition_token,
                close_condition_token,
            }
        },
    )(i)
}

fn do_while_statement(i: TokenList) -> IResult<TokenList, DoWhileStatement> {
    map(
        pair(
            terminal(TerminalToken::Do),
            perfect(
                "do-while loop",
                tuple((
                    inline_statement,
                    terminal(TerminalToken::While),
                    terminal(TerminalToken::OpenBracket),
                    expression,
                    terminal(TerminalToken::CloseBracket),
                )),
            ),
        ),
        |(
            do_token,
            (body, while_token, open_condition_token, condition, close_condition_token),
        )| DoWhileStatement {
            body: Box::new(body),
            condition: Box::new(condition),

            do_token,
            while_token,
            open_condition_token,
            close_condition_token,
        },
    )(i)
}

fn switch_case(i: TokenList) -> IResult<TokenList, SwitchCase> {
    let condition = alt((
        map(
            pair(
                terminal(TerminalToken::Case),
                perfect("switch case condition", expression),
            ),
            |(case_token, expr)| SwitchCaseCondition::Expr {
                expr: Box::new(expr),
                case_token,
            },
        ),
        map(terminal(TerminalToken::Default), |default_token| {
            SwitchCaseCondition::Default { default_token }
        }),
    ));

    map(
        tuple((
            condition,
            terminal(TerminalToken::Colon),
            perfect("switch case body", statement_list),
        )),
        |(condition, end_condition_token, body)| SwitchCase {
            condition,
            body,

            end_condition_token,
        },
    )(i)
}

fn switch_statement(i: TokenList) -> IResult<TokenList, SwitchStatement> {
    map(
        pair(
            terminal(TerminalToken::Switch),
            perfect(
                "switch statement",
                tuple((
                    terminal(TerminalToken::OpenBracket),
                    expression,
                    terminal(TerminalToken::CloseBracket),
                    terminal(TerminalToken::OpenBrace),
                    many_till(
                        perfect("switch case", switch_case),
                        terminal(TerminalToken::CloseBrace),
                    ),
                )),
            ),
        ),
        |(
            switch_token,
            (
                open_condition_token,
                condition,
                close_condition_token,
                open_cases_token,
                (cases, close_cases_token),
            ),
        )| SwitchStatement {
            condition: Box::new(condition),
            cases,

            switch_token,
            open_condition_token,
            close_condition_token,
            open_cases_token,
            close_cases_token,
        },
    )(i)
}

fn expression_or_declaration(i: TokenList) -> IResult<TokenList, ExpressionOrDeclaration> {
    alt((
        map(
            var_declaration_statement,
            ExpressionOrDeclaration::Declaration,
        ),
        map(expression, |expr| {
            ExpressionOrDeclaration::Expression(Box::new(expr))
        }),
    ))(i)
}

fn for_statement(i: TokenList) -> IResult<TokenList, ForStatement> {
    map(
        pair(
            terminal(TerminalToken::For),
            perfect(
                "for loop",
                tuple((
                    terminal(TerminalToken::OpenBracket),
                    context(
                        "for loop condition",
                        tuple((
                            opt(expression_or_declaration),
                            terminal(TerminalToken::Semicolon),
                            opt(expression),
                            terminal(TerminalToken::Semicolon),
                            opt(expression),
                        )),
                    ),
                    terminal(TerminalToken::CloseBracket),
                    inline_statement,
                )),
            ),
        ),
        |(
            for_token,
            (
                open_header_token,
                (initializer, first_separator_token, condition, second_separator_token, increment),
                close_header_token,
                body,
            ),
        )| ForStatement {
            initializer,
            condition: condition.map(Box::new),
            increment: increment.map(Box::new),
            body: Box::new(body),

            for_token,
            open_header_token,
            close_header_token,
            first_separator_token,
            second_separator_token,
        },
    )(i)
}

fn foreach_index(i: TokenList) -> IResult<TokenList, ForeachIndex> {
    alt((
        map(
            tuple((ty(true), identifier, terminal(TerminalToken::Comma))),
            |(name_type, name, separator_token)| ForeachIndex {
                name_type: Some(name_type),
                name,
                separator_token,
            },
        ),
        map(
            pair(identifier, terminal(TerminalToken::Comma)),
            |(name, separator_token)| ForeachIndex {
                name_type: None,
                name,
                separator_token,
            },
        ),
    ))(i)
}

fn foreach_statement(i: TokenList) -> IResult<TokenList, ForeachStatement> {
    map(
        pair(
            terminal(TerminalToken::Foreach),
            perfect(
                "foreach loop",
                tuple((
                    terminal(TerminalToken::OpenBracket),
                    perfect(
                        "foreach loop condition",
                        tuple((
                            opt(foreach_index),
                            alt((
                                map(
                                    tuple((ty(true), identifier, terminal(TerminalToken::In))),
                                    |(value_type, value_name, separator_token)| {
                                        (Some(value_type), value_name, separator_token)
                                    },
                                ),
                                map(
                                    pair(identifier, terminal(TerminalToken::In)),
                                    |(value_name, separator_token)| {
                                        (None, value_name, separator_token)
                                    },
                                ),
                            )),
                            expression,
                        )),
                    ),
                    terminal(TerminalToken::CloseBracket),
                    inline_statement,
                )),
            ),
        ),
        |(
            foreach_token,
            (
                open_header_token,
                (index, (value_type, value_name, separator_token), array),
                close_header_token,
                body,
            ),
        )| ForeachStatement {
            index,
            value_type,
            value_name,
            array: Box::new(array),
            body: Box::new(body),

            foreach_token,
            open_header_token,
            close_header_token,
            separator_token,
        },
    )(i)
}

fn break_statement(i: TokenList) -> IResult<TokenList, BreakStatement> {
    map(terminal(TerminalToken::Break), |token| BreakStatement {
        token,
    })(i)
}

fn continue_statement(i: TokenList) -> IResult<TokenList, ContinueStatement> {
    map(terminal(TerminalToken::Continue), |token| {
        ContinueStatement { token }
    })(i)
}

fn return_statement(i: TokenList) -> IResult<TokenList, ReturnStatement> {
    alt((
        map(
            pair(not_line_ending(terminal(TerminalToken::Return)), expression),
            |(return_token, value)| ReturnStatement {
                value: Some(Box::new(value)),
                return_token,
            },
        ),
        map(terminal(TerminalToken::Return), |return_token| {
            ReturnStatement {
                value: None,
                return_token,
            }
        }),
    ))(i)
}

fn yield_statement(i: TokenList) -> IResult<TokenList, YieldStatement> {
    alt((
        map(
            pair(not_line_ending(terminal(TerminalToken::Yield)), expression),
            |(yield_token, value)| YieldStatement {
                value: Some(Box::new(value)),
                yield_token,
            },
        ),
        map(terminal(TerminalToken::Yield), |yield_token| {
            YieldStatement {
                value: None,
                yield_token,
            }
        }),
    ))(i)
}

fn class_declaration_statement(i: TokenList) -> IResult<TokenList, ClassDeclarationStatement> {
    map(
        tuple((
            terminal(TerminalToken::Class),
            context("class declaration name", expression),
            class_declaration,
        )),
        |(class_token, name, class)| ClassDeclarationStatement {
            name: Box::new(name),
            class,

            class_token,
        },
    )(i)
}

fn try_catch_statement(i: TokenList) -> IResult<TokenList, TryCatchStatement> {
    map(
        tuple((
            terminal(TerminalToken::Try),
            perfect(
                "try statement",
                tuple((
                    inline_statement,
                    terminal(TerminalToken::Catch),
                    terminal(TerminalToken::OpenBracket),
                    context("catch name", identifier),
                    terminal(TerminalToken::CloseBracket),
                    context("catch statement", inline_statement),
                )),
            ),
        )),
        |(
            try_token,
            (
                body,
                catch_token,
                open_catch_name_token,
                catch_name,
                close_catch_name_token,
                catch_body,
            ),
        )| TryCatchStatement {
            body: Box::new(body),
            catch_name,
            catch_body: Box::new(catch_body),

            try_token,
            catch_token,
            open_catch_name_token,
            close_catch_name_token,
        },
    )(i)
}

fn throw_statement(i: TokenList) -> IResult<TokenList, ThrowStatement> {
    map(
        pair(
            terminal(TerminalToken::Throw),
            perfect("throw statement", expression),
        ),
        |(throw_token, value)| ThrowStatement {
            value: Box::new(value),
            throw_token,
        },
    )(i)
}

fn const_statement(i: TokenList) -> IResult<TokenList, ConstStatement> {
    map(
        pair(
            terminal(TerminalToken::Const),
            perfect(
                "const statement",
                tuple((
                    alt((
                        map(pair(ty(false), identifier), |(ty, identifier)| {
                            (Some(ty), identifier)
                        }),
                        map(identifier, |identifier| (None, identifier)),
                    )),
                    terminal(TerminalToken::Assign),
                    perfect("const initializer", expression),
                )),
            ),
        ),
        |(const_token, ((const_type, name), separator_token, value))| ConstStatement {
            const_type,
            name,
            value: Box::new(value),

            const_token,
            separator_token,
        },
    )(i)
}

fn enum_entry(i: TokenList) -> IResult<TokenList, EnumEntry> {
    map(
        tuple((
            identifier,
            opt(var_initializer),
            opt(terminal(TerminalToken::Comma)),
        )),
        |(name, initializer, separator_token)| EnumEntry {
            name,
            initializer,

            separator_token,
        },
    )(i)
}

fn enum_statement(i: TokenList) -> IResult<TokenList, EnumStatement> {
    map(
        pair(
            terminal(TerminalToken::Enum),
            perfect(
                "enum statement",
                tuple((
                    identifier,
                    terminal(TerminalToken::OpenBrace),
                    many_till(
                        perfect("enum entry", enum_entry),
                        terminal(TerminalToken::CloseBrace),
                    ),
                )),
            ),
        ),
        |(enum_token, (name, open_entries_token, (entries, close_entries_token)))| EnumStatement {
            name,
            entries,

            enum_token,
            open_entries_token,
            close_entries_token,
        },
    )(i)
}

fn var_or_func_declaration_statement(i: TokenList) -> IResult<TokenList, StatementType> {
    let void_func = map(
        tuple((
            terminal(TerminalToken::Function),
            separated_list1(TerminalToken::Namespace, identifier),
            perfect("function declaration", function_declaration),
        )),
        |(function_token, name, function)| {
            StatementType::FunctionDeclaration(FunctionDeclarationStatement {
                return_type: None,
                name,
                function,

                function_token,
            })
        },
    );

    alt((void_func, typed_declaration_statement))(i)
}

fn typed_declaration_statement(i: TokenList) -> IResult<TokenList, StatementType> {
    let (i, declaration_ty) = ty(false)(i)?;
    let (i, maybe_function_token) = opt(terminal(TerminalToken::Function))(i)?;

    match maybe_function_token {
        Some(function_token) => {
            let (i, (name, function)) = pair(
                separated_list1(TerminalToken::Namespace, identifier),
                perfect("function declaration", function_declaration),
            )(i)?;
            Ok((
                i,
                StatementType::FunctionDeclaration(FunctionDeclarationStatement {
                    return_type: Some(declaration_ty),
                    name,
                    function,

                    function_token,
                }),
            ))
        }
        None => map(
            var_declaration_statement_with_type(declaration_ty),
            StatementType::VarDeclaration,
        )(i),
    }
}

fn var_declaration(i: TokenList) -> IResult<TokenList, VarDeclaration> {
    map(
        pair(identifier, opt(var_initializer)),
        |(name, initializer)| VarDeclaration { name, initializer },
    )(i)
}

fn var_declaration_statement_with_type<'s>(
    var_type: Type<'s>,
) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, VarDeclarationStatement<'s>> {
    map(
        pair(
            separated_list1(TerminalToken::Comma, var_declaration),
            opt(terminal(TerminalToken::Comma)),
        ),
        move |(declarations, trailing_separator_token)| VarDeclarationStatement {
            var_type: var_type.clone(),
            declarations,
            trailing_separator_token,
        },
    )
}

fn var_declaration_statement(i: TokenList) -> IResult<TokenList, VarDeclarationStatement> {
    let (i, var_type) = ty(false)(i)?;
    var_declaration_statement_with_type(var_type)(i)
}

fn thread_statement(i: TokenList) -> IResult<TokenList, ThreadStatement> {
    map(
        pair(
            terminal(TerminalToken::Thread),
            perfect("thread statement", expression),
        ),
        |(thread_token, value)| ThreadStatement {
            value: Box::new(value),

            thread_token,
        },
    )(i)
}

fn delay_thread_statement(i: TokenList) -> IResult<TokenList, DelayThreadStatement> {
    map(
        pair(
            terminal(TerminalToken::DelayThread),
            perfect(
                "delaythread statement",
                tuple((
                    terminal(TerminalToken::OpenBracket),
                    expression,
                    terminal(TerminalToken::CloseBracket),
                    expression,
                )),
            ),
        ),
        |(delay_thread_token, (open_duration_token, duration, close_duration_token, value))| {
            DelayThreadStatement {
                duration: Box::new(duration),
                value: Box::new(value),

                delay_thread_token,
                open_duration_token,
                close_duration_token,
            }
        },
    )(i)
}

fn wait_thread_statement(i: TokenList) -> IResult<TokenList, WaitThreadStatement> {
    map(
        pair(
            terminal(TerminalToken::WaitThread),
            perfect("waitthread statement", expression),
        ),
        |(wait_thread_token, value)| WaitThreadStatement {
            value: Box::new(value),

            wait_thread_token,
        },
    )(i)
}

fn wait_statement(i: TokenList) -> IResult<TokenList, WaitStatement> {
    map(
        pair(terminal(TerminalToken::Wait), perfect("wait", expression)),
        |(wait_token, value)| WaitStatement {
            value: Box::new(value),
            wait_token,
        },
    )(i)
}

fn struct_declaration_statement(i: TokenList) -> IResult<TokenList, StructDeclarationStatement> {
    map(
        tuple((
            terminal(TerminalToken::Struct),
            identifier,
            perfect("struct declaration", struct_declaration),
        )),
        |(struct_token, name, declaration)| StructDeclarationStatement {
            name,
            declaration,

            struct_token,
        },
    )(i)
}

fn type_declaration_statement(i: TokenList) -> IResult<TokenList, TypeDeclarationStatement> {
    map(
        pair(
            terminal(TerminalToken::Typedef),
            perfect("type declaration", pair(identifier, ty(true))),
        ),
        |(typedef_token, (name, base_type))| TypeDeclarationStatement {
            name,
            base_type,

            typedef_token,
        },
    )(i)
}

fn global_type(i: TokenList) -> IResult<TokenList, GlobalType> {
    let global_function = map(
        pair(
            terminal(TerminalToken::Function),
            perfect(
                "global function declaration",
                separated_list1(TerminalToken::Namespace, identifier),
            ),
        ),
        |(function_token, name)| GlobalType::Function {
            name,
            function_token,
        },
    );
    let global_var_declaration = map(var_declaration_statement, GlobalType::VarDeclaration);

    alt((
        global_function,
        global_var_declaration,
        map(const_statement, GlobalType::Const),
        map(enum_statement, GlobalType::Enum),
        map(class_declaration_statement, GlobalType::Class),
        map(struct_declaration_statement, GlobalType::Struct),
        map(type_declaration_statement, GlobalType::Type),
    ))(i)
}

fn global_statement(i: TokenList) -> IResult<TokenList, GlobalStatement> {
    map(
        pair(
            terminal(TerminalToken::Global),
            perfect("global declaration", global_type),
        ),
        |(global_token, ty)| GlobalStatement { ty, global_token },
    )(i)
}

fn globalize_all_functions_statement(
    i: TokenList,
) -> IResult<TokenList, GlobalizeAllFunctionsStatement> {
    map(terminal(TerminalToken::GlobalizeAllFunctions), |token| {
        GlobalizeAllFunctionsStatement { token }
    })(i)
}

fn untyped_statement(i: TokenList) -> IResult<TokenList, UntypedStatement> {
    map(terminal(TerminalToken::Untyped), |token| UntypedStatement {
        token,
    })(i)
}
