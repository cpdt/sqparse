use crate::ast::{
    ArrayExpression, BinaryExpression, CallExpression, ClassExpression, CommaExpression,
    DelegateExpression, ExpectExpression, Expression, FunctionExpression, IndexExpression,
    LiteralExpression, ParensExpression, PostfixExpression, Precedence, PrefixExpression,
    PropertyExpression, RootVarExpression, TableExpression, TernaryExpression, VarExpression,
    VectorExpression,
};
use crate::parser::array::array_value;
use crate::parser::class::class_definition;
use crate::parser::function::function_definition;
use crate::parser::identifier::{identifier, method_identifier};
use crate::parser::operator::{binary_operator, postfix_operator, prefix_operator};
use crate::parser::parse_result_ext::ParseResultExt;
use crate::parser::table::table_slot;
use crate::parser::token_list::TokenList;
use crate::parser::token_list_ext::TokenListExt;
use crate::parser::type_::type_;
use crate::parser::ParseResult;
use crate::token::{TerminalToken, TokenType};
use crate::{ContextType, ParseErrorType};

pub fn expression(tokens: TokenList, precedence: Precedence) -> ParseResult<Box<Expression>> {
    let (mut next_tokens, mut value) = value(tokens)?;

    loop {
        let mut value_container = Some(value);
        match operator(next_tokens, precedence, ExpressionRef(&mut value_container))
            .with_context_from(ContextType::Expression, tokens)
            .maybe(next_tokens)?
        {
            (new_tokens, Some(new_value)) => {
                next_tokens = new_tokens;
                value = new_value;
            }
            (new_tokens, None) => return Ok((new_tokens, value_container.unwrap())),
        }
    }
}

fn value(tokens: TokenList) -> ParseResult<Box<Expression>> {
    function(tokens)
        .map_val(Expression::Function)
        .or_try(|| parens(tokens).map_val(Expression::Parens))
        .or_try(|| literal(tokens).map_val(Expression::Literal))
        .or_try(|| var(tokens).map_val(Expression::Var))
        .or_try(|| root_var(tokens).map_val(Expression::RootVar))
        .or_try(|| table(tokens).map_val(Expression::Table))
        .or_try(|| class(tokens).map_val(Expression::Class))
        .or_try(|| array(tokens).map_val(Expression::Array))
        .or_try(|| vector(tokens).map_val(Expression::Vector))
        .or_try(|| prefix(tokens).map_val(Expression::Prefix))
        .or_try(|| delegate(tokens).map_val(Expression::Delegate))
        .or_try(|| expect(tokens).map_val(Expression::Expect))
        .or_error(|| tokens.error(ParseErrorType::ExpectedValue))
        .map_val(Box::new)
}

pub fn function(tokens: TokenList) -> ParseResult<FunctionExpression> {
    type_(tokens)
        .not_line_ending()
        .maybe(tokens)
        .and_then(|(tokens, return_type)| {
            tokens
                .terminal(TerminalToken::Function)
                .map_val(|function| (return_type, function))
        })
        .determines(|tokens, (return_type, function)| {
            function_definition(tokens).map_val(|definition| FunctionExpression {
                return_type,
                function,
                definition,
            })
        })
        .with_context_from(ContextType::FunctionLiteral, tokens)
}

pub fn parens(tokens: TokenList) -> ParseResult<ParensExpression> {
    tokens
        .terminal(TerminalToken::OpenBracket)
        .determines_and_opens(
            ContextType::Expression,
            |tokens| tokens.terminal(TerminalToken::CloseBracket),
            |tokens, open, close| {
                expression(tokens, Precedence::None).map_val(|value| ParensExpression {
                    open,
                    value,
                    close,
                })
            },
        )
}

pub fn literal(tokens: TokenList) -> ParseResult<LiteralExpression> {
    if let Some((tokens, item)) = tokens.split_first() {
        if let TokenType::Literal(literal) = item.token.ty {
            return Ok((
                tokens,
                LiteralExpression {
                    literal,
                    token: &item.token,
                },
            ));
        }
    }

    Err(tokens.error(ParseErrorType::ExpectedLiteral))
}

pub fn var(tokens: TokenList) -> ParseResult<VarExpression> {
    identifier(tokens).map_val(|name| VarExpression { name })
}

pub fn root_var(tokens: TokenList) -> ParseResult<RootVarExpression> {
    tokens
        .terminal(TerminalToken::Namespace)
        .determines(|tokens, root| {
            identifier(tokens).map_val(|name| RootVarExpression { root, name })
        })
}

pub fn table(tokens: TokenList) -> ParseResult<TableExpression> {
    table_delimited(tokens, TerminalToken::OpenBrace, TerminalToken::CloseBrace)
}

pub fn table_delimited(
    tokens: TokenList,
    open_terminal: TerminalToken,
    close_terminal: TerminalToken,
) -> ParseResult<TableExpression> {
    tokens.terminal(open_terminal).determines_and_opens(
        ContextType::TableLiteral,
        |tokens| tokens.terminal(close_terminal),
        |tokens, open, close| {
            let (tokens, slots) = tokens.many_until(
                |tokens| tokens.is_ended() || tokens.terminal(TerminalToken::Ellipsis).is_ok(),
                table_slot,
            )?;
            let (tokens, spread) = tokens.terminal(TerminalToken::Ellipsis).maybe(tokens)?;
            Ok((
                tokens,
                TableExpression {
                    open,
                    slots,
                    spread,
                    close,
                },
            ))
        },
    )
}

pub fn class(tokens: TokenList) -> ParseResult<ClassExpression> {
    tokens
        .terminal(TerminalToken::Class)
        .determines(|tokens, class| {
            class_definition(tokens).map_val(|definition| ClassExpression { class, definition })
        })
        .with_context_from(ContextType::ClassLiteral, tokens)
}

pub fn array(tokens: TokenList) -> ParseResult<ArrayExpression> {
    tokens
        .terminal(TerminalToken::OpenSquare)
        .determines_and_opens(
            ContextType::ArrayLiteral,
            |tokens| tokens.terminal(TerminalToken::CloseSquare),
            |tokens, open, close| {
                let (tokens, values) = tokens.many_until(
                    |tokens| tokens.is_ended() || tokens.terminal(TerminalToken::Ellipsis).is_ok(),
                    array_value,
                )?;
                let (tokens, spread) = tokens.terminal(TerminalToken::Ellipsis).maybe(tokens)?;
                Ok((
                    tokens,
                    ArrayExpression {
                        open,
                        values,
                        spread,
                        close,
                    },
                ))
            },
        )
}

pub fn vector(tokens: TokenList) -> ParseResult<VectorExpression> {
    tokens
        .terminal(TerminalToken::Less)
        .determines(|tokens, open| {
            let (tokens, x) = expression(tokens, Precedence::Comma)?;
            let (tokens, comma_1) = tokens.terminal(TerminalToken::Comma)?;
            let (tokens, y) = expression(tokens, Precedence::Comma)?;
            let (tokens, comma_2) = tokens.terminal(TerminalToken::Comma)?;
            let (tokens, z) = expression(tokens, Precedence::Bitshift)?;
            let (tokens, close) = tokens.terminal(TerminalToken::Greater)?;
            Ok((
                tokens,
                VectorExpression {
                    open,
                    x,
                    comma_1,
                    y,
                    comma_2,
                    z,
                    close,
                },
            ))
        })
        .with_context_from(ContextType::VectorLiteral, tokens)
}

pub fn prefix(tokens: TokenList) -> ParseResult<PrefixExpression> {
    prefix_operator(tokens).determines(|tokens, operator| {
        expression(tokens, Precedence::Prefix).map_val(|value| PrefixExpression { operator, value })
    })
}

pub fn delegate(tokens: TokenList) -> ParseResult<DelegateExpression> {
    tokens
        .terminal(TerminalToken::Delegate)
        .determines(|tokens, delegate| {
            let (tokens, parent) = expression(tokens, Precedence::None)?;
            let (tokens, colon) = tokens.terminal(TerminalToken::Colon)?;
            let (tokens, value) = expression(tokens, Precedence::Comma)?;
            Ok((
                tokens,
                DelegateExpression {
                    delegate,
                    parent,
                    colon,
                    value,
                },
            ))
        })
}

pub fn expect(tokens: TokenList) -> ParseResult<ExpectExpression> {
    tokens
        .terminal(TerminalToken::Expect)
        .determines(|tokens, expect| {
            let (tokens, ty) = type_(tokens)?;
            tokens.terminal(TerminalToken::OpenBracket).opens(
                ContextType::Expression,
                |tokens| tokens.terminal(TerminalToken::CloseBracket),
                |tokens, open, close| {
                    let (tokens, value) = expression(tokens, Precedence::None)?;
                    Ok((
                        tokens,
                        ExpectExpression {
                            expect,
                            ty,
                            open,
                            value,
                            close,
                        },
                    ))
                },
            )
        })
}

struct ExpressionRef<'a, 's>(&'a mut Option<Box<Expression<'s>>>);
impl<'a, 's> ExpressionRef<'a, 's> {
    fn take(self) -> Box<Expression<'s>> {
        self.0.take().unwrap()
    }
}

fn operator<'s>(
    tokens: TokenList<'s>,
    precedence: Precedence,
    left: ExpressionRef<'_, 's>,
) -> ParseResult<'s, Box<Expression<'s>>> {
    let left_ref = left.0;

    property(tokens, precedence, ExpressionRef(left_ref))
        .map_val(Expression::Property)
        .or_try(|| {
            ternary(tokens, precedence, ExpressionRef(left_ref)).map_val(Expression::Ternary)
        })
        .or_try(|| binary(tokens, precedence, ExpressionRef(left_ref)).map_val(Expression::Binary))
        .or_try(|| index(tokens, precedence, ExpressionRef(left_ref)).map_val(Expression::Index))
        .or_try(|| {
            postfix(tokens, precedence, ExpressionRef(left_ref)).map_val(Expression::Postfix)
        })
        .or_try(|| call(tokens, precedence, ExpressionRef(left_ref)).map_val(Expression::Call))
        .or_try(|| comma(tokens, precedence, ExpressionRef(left_ref)).map_val(Expression::Comma))
        .or_error(|| tokens.error(ParseErrorType::ExpectedOperator))
        .map_val(Box::new)
}

fn property<'s>(
    tokens: TokenList<'s>,
    precedence: Precedence,
    left: ExpressionRef<'_, 's>,
) -> ParseResult<'s, PropertyExpression<'s>> {
    // left associative
    if precedence >= Precedence::Property {
        return Err(tokens.error(ParseErrorType::Precedence));
    }

    tokens
        .terminal(TerminalToken::Dot)
        .determines(|tokens, dot| {
            method_identifier(tokens).map_val(|property| PropertyExpression {
                base: left.take(),
                dot,
                property,
            })
        })
}

fn ternary<'s>(
    tokens: TokenList<'s>,
    precedence: Precedence,
    left: ExpressionRef<'_, 's>,
) -> ParseResult<'s, TernaryExpression<'s>> {
    // right associative
    if precedence > Precedence::Ternary {
        return Err(tokens.error(ParseErrorType::Precedence));
    }

    tokens
        .terminal(TerminalToken::Question)
        .determines(|tokens, question| {
            let (tokens, true_value) = expression(tokens, Precedence::None)?;
            let (tokens, separator) = tokens.terminal(TerminalToken::Colon)?;
            let (tokens, false_value) = expression(tokens, Precedence::Ternary)?;

            Ok((
                tokens,
                TernaryExpression {
                    condition: left.take(),
                    question,
                    true_value,
                    separator,
                    false_value,
                },
            ))
        })
}

fn binary<'s>(
    tokens: TokenList<'s>,
    precedence: Precedence,
    left: ExpressionRef<'_, 's>,
) -> ParseResult<'s, BinaryExpression<'s>> {
    binary_operator(tokens)
        .and_then(|(tokens, operator)| {
            // left associative
            if precedence >= operator.precedence() {
                Err(tokens.error(ParseErrorType::Precedence))
            } else {
                Ok((tokens, operator))
            }
        })
        .determines(|tokens, operator| {
            expression(tokens, operator.precedence()).map_val(|right| BinaryExpression {
                left: left.take(),
                operator,
                right,
            })
        })
}

fn index<'s>(
    tokens: TokenList<'s>,
    precedence: Precedence,
    left: ExpressionRef<'_, 's>,
) -> ParseResult<'s, IndexExpression<'s>> {
    // left associative
    if precedence >= Precedence::Postfix {
        return Err(tokens.error(ParseErrorType::Precedence));
    }

    tokens
        .terminal(TerminalToken::OpenSquare)
        .determines_and_opens(
            ContextType::Expression,
            |tokens| tokens.terminal(TerminalToken::CloseSquare),
            |tokens, open, close| {
                expression(tokens, Precedence::None).map_val(|index| IndexExpression {
                    base: left.take(),
                    open,
                    index,
                    close,
                })
            },
        )
}

fn postfix<'s>(
    tokens: TokenList<'s>,
    precedence: Precedence,
    left: ExpressionRef<'_, 's>,
) -> ParseResult<'s, PostfixExpression<'s>> {
    // left associative
    if precedence >= Precedence::Postfix {
        return Err(tokens.error(ParseErrorType::Precedence));
    }

    // Newlines are not allowed before postfix operators to prevent this:
    // ```
    // a
    // ++b
    // ```
    // from being parsed as:
    // ```
    // (a++) b
    // ```
    if tokens.is_newline() {
        return Err(tokens.error(ParseErrorType::IllegalLineBreak));
    }

    postfix_operator(tokens)
        .not_definite()
        .map_val(|operator| PostfixExpression {
            value: left.take(),
            operator,
        })
}

fn call<'s>(
    tokens: TokenList<'s>,
    precedence: Precedence,
    left: ExpressionRef<'_, 's>,
) -> ParseResult<'s, CallExpression<'s>> {
    // left associative
    if precedence >= Precedence::Postfix {
        return Err(tokens.error(ParseErrorType::Precedence));
    }

    tokens
        .terminal(TerminalToken::OpenBracket)
        .determines_and_opens(
            ContextType::CallArgumentList,
            |tokens| tokens.terminal(TerminalToken::CloseBracket),
            |tokens, open, close| {
                tokens
                    .separated_list_trailing0(
                        |tokens| {
                            let res = expression(tokens, Precedence::Comma).map_val(|expr| *expr);
                            if tokens.is_ended() {
                                res
                            } else {
                                res.definite()
                            }
                        },
                        |tokens| tokens.terminal(TerminalToken::Comma),
                    )
                    .map_val(|args| (open, args, close))
            },
        )
        .and_then(|(tokens, (open, arguments, close))| {
            // Post-initializer may appear after a call, as long as it is on the same line.
            let (tokens, post_initializer) = if tokens.is_newline() {
                (tokens, None)
            } else {
                table(tokens).maybe(tokens)?
            };

            Ok((
                tokens,
                CallExpression {
                    function: left.take(),
                    open,
                    arguments,
                    close,
                    post_initializer,
                },
            ))
        })
}

fn comma<'s>(
    tokens: TokenList<'s>,
    precedence: Precedence,
    left: ExpressionRef<'_, 's>,
) -> ParseResult<'s, CommaExpression<'s>> {
    // left associative
    if precedence >= Precedence::Comma {
        return Err(tokens.error(ParseErrorType::Precedence));
    }

    tokens
        .terminal(TerminalToken::Comma)
        .determines(|tokens, first_comma| {
            let (tokens, mut values) = tokens.separated_list1(
                |tokens| expression(tokens, Precedence::Comma).map_val(|expr| *expr),
                |tokens| tokens.terminal(TerminalToken::Comma),
            )?;
            values.items.insert(0, (*left.take(), first_comma));
            Ok((tokens, CommaExpression { values }))
        })
}
