use crate::ast::{
    ArrayExpression, BinaryExpression, CallExpression, ClassExpression, CommaExpression,
    DelegateExpression, ExpectExpression, Expression, FunctionExpression, IndexExpression,
    LiteralExpression, ParensExpression, PostfixExpression, Precedence, PrefixExpression,
    PropertyExpression, RootVarExpression, TableExpression, TernaryExpression, VarExpression,
    VectorExpression,
};
use crate::parser::array::array_value;
use crate::parser::class::class_declaration;
use crate::parser::combinator::{
    alt_map, definitely, first_of, map, opt, prevent_ending_line, span,
};
use crate::parser::error::InternalErrorType;
use crate::parser::function::function_declaration;
use crate::parser::identifier::{identifier, method_identifier};
use crate::parser::list::{many, separated_list1, separated_list_trailing0};
use crate::parser::operator::{binary_operator, postfix_operator, prefix_operator};
use crate::parser::table::table_slot;
use crate::parser::token::terminal;
use crate::parser::type_::type_;
use crate::parser::{ContextType, ParseError, ParseErrorType, ParseResult, TokenList};
use crate::token::{TerminalToken, TokenType};

pub fn expression(tokens: TokenList, precedence: Precedence) -> ParseResult<Expression> {
    let (mut tokens, mut value) = value(tokens)?;

    loop {
        let mut value_container = Some(value);
        match operator(tokens, precedence, ExpressionRef(&mut value_container)) {
            Ok((new_tokens, new_value)) => {
                tokens = new_tokens;
                value = new_value;
            }
            Err(err) if err.is_fatal => return Err(err),
            Err(_) => return Ok((tokens, value_container.unwrap())),
        }
    }
}

pub fn value(tokens: TokenList) -> ParseResult<Expression> {
    first_of(
        tokens,
        [
            // Must be before other value types to ensure the return type is parsed.
            |tokens| map(function(tokens), Expression::Function),
            |tokens| map(parens(tokens), Expression::Parens),
            |tokens| map(literal(tokens), Expression::Literal),
            |tokens| map(var(tokens), Expression::Var),
            |tokens| map(root_var(tokens), Expression::RootVar),
            |tokens| map(prefix(tokens), Expression::Prefix),
            |tokens| map(table(tokens), Expression::Table),
            |tokens| map(class(tokens), Expression::Class),
            |tokens| map(array(tokens), Expression::Array),
            |tokens| map(delegate(tokens), Expression::Delegate),
            |tokens| map(vector(tokens), Expression::Vector),
            |tokens| map(expect(tokens), Expression::Expect),
        ],
        |_| {
            Err(ParseError::new(
                ParseErrorType::ExpectedExpression,
                tokens.start_index(),
            ))
        },
    )
}

pub fn parens(tokens: TokenList) -> ParseResult<ParensExpression> {
    span(
        tokens,
        ContextType::ParensExpression,
        TerminalToken::OpenBracket,
        TerminalToken::CloseBracket,
        |tokens, open, close| {
            let (tokens, value) = expression(tokens, Precedence::None)?;
            Ok((
                tokens,
                ParensExpression {
                    open,
                    value: Box::new(value),
                    close,
                },
            ))
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

    Err(ParseError::new(
        ParseErrorType::ExpectedLiteral,
        tokens.start_index(),
    ))
}

pub fn var(tokens: TokenList) -> ParseResult<VarExpression> {
    let (tokens, name) = identifier(tokens)?;
    Ok((tokens, VarExpression { name }))
}

pub fn root_var(tokens: TokenList) -> ParseResult<RootVarExpression> {
    definitely(
        tokens,
        ContextType::RootVarExpression,
        |tokens| terminal(tokens, TerminalToken::Namespace),
        |tokens, root| {
            let (tokens, name) = identifier(tokens)?;

            Ok((tokens, RootVarExpression { root, name }))
        },
    )
}

pub fn prefix(tokens: TokenList) -> ParseResult<PrefixExpression> {
    definitely(
        tokens,
        ContextType::ExpressionRightHandSide,
        prefix_operator,
        |tokens, operator| {
            let (tokens, value) = expression(tokens, Precedence::Prefix)?;

            Ok((
                tokens,
                PrefixExpression {
                    operator,
                    value: Box::new(value),
                },
            ))
        },
    )
}

pub fn table(tokens: TokenList) -> ParseResult<TableExpression> {
    table_delimited(tokens, TerminalToken::OpenBrace, TerminalToken::CloseBrace)
}

pub fn table_delimited(
    tokens: TokenList,
    open_terminal: TerminalToken,
    close_terminal: TerminalToken,
) -> ParseResult<TableExpression> {
    span(
        tokens,
        ContextType::TableExpression,
        open_terminal,
        close_terminal,
        |tokens, open, close| {
            let (tokens, slots) = many(tokens, table_slot)?;
            let (tokens, spread) = opt(tokens, terminal(tokens, TerminalToken::Ellipsis))?;
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
    definitely(
        tokens,
        ContextType::ClassExpression,
        |tokens| terminal(tokens, TerminalToken::Class),
        |tokens, class| {
            let (tokens, declaration) = class_declaration(tokens)?;
            Ok((tokens, ClassExpression { class, declaration }))
        },
    )
}

pub fn array(tokens: TokenList) -> ParseResult<ArrayExpression> {
    span(
        tokens,
        ContextType::ArrayExpression,
        TerminalToken::OpenSquare,
        TerminalToken::CloseSquare,
        |tokens, open, close| {
            let (tokens, values) = many(tokens, array_value)?;
            let (tokens, spread) = opt(tokens, terminal(tokens, TerminalToken::Ellipsis))?;
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

pub fn delegate(tokens: TokenList) -> ParseResult<DelegateExpression> {
    definitely(
        tokens,
        ContextType::DelegateExpression,
        |tokens| terminal(tokens, TerminalToken::Delegate),
        |tokens, delegate| {
            let (tokens, parent) = expression(tokens, Precedence::None)?;
            let (tokens, colon) = terminal(tokens, TerminalToken::Colon)?;
            let (tokens, value) = expression(tokens, Precedence::Comma)?;

            Ok((
                tokens,
                DelegateExpression {
                    delegate,
                    parent: Box::new(parent),
                    colon,
                    value: Box::new(value),
                },
            ))
        },
    )
}

pub fn vector(tokens: TokenList) -> ParseResult<VectorExpression> {
    definitely(
        tokens,
        ContextType::VectorExpression,
        |tokens| terminal(tokens, TerminalToken::Less),
        |tokens, open| {
            let (tokens, x) = expression(tokens, Precedence::Comma)?;
            let (tokens, comma_1) = terminal(tokens, TerminalToken::Comma)?;
            let (tokens, y) = expression(tokens, Precedence::Comma)?;
            let (tokens, comma_2) = terminal(tokens, TerminalToken::Comma)?;
            let (tokens, z) = expression(tokens, Precedence::Bitshift)?;
            let (tokens, close) = terminal(tokens, TerminalToken::Greater)?;

            Ok((
                tokens,
                VectorExpression {
                    open,
                    x: Box::new(x),
                    comma_1,
                    y: Box::new(y),
                    comma_2,
                    z: Box::new(z),
                    close,
                },
            ))
        },
    )
}

pub fn expect(tokens: TokenList) -> ParseResult<ExpectExpression> {
    definitely(
        tokens,
        ContextType::ExpectExpression,
        |tokens| terminal(tokens, TerminalToken::Expect),
        |tokens, expect| {
            let (tokens, ty) = type_(tokens)?;
            span(
                tokens,
                ContextType::ExpectExpression,
                TerminalToken::OpenBracket,
                TerminalToken::CloseBracket,
                |tokens, open, close| {
                    let (tokens, value) = expression(tokens, Precedence::None)?;
                    Ok((
                        tokens,
                        ExpectExpression {
                            expect,
                            ty,
                            open,
                            value: Box::new(value),
                            close,
                        },
                    ))
                },
            )
        },
    )
}

pub fn function(tokens: TokenList) -> ParseResult<FunctionExpression> {
    definitely(
        tokens,
        ContextType::FunctionExpression,
        |tokens| {
            let (tokens, return_type) = prevent_ending_line(
                tokens,
                opt(tokens, type_(tokens).map_err(|err| err.into_non_fatal())),
            )?;
            let (tokens, function) = terminal(tokens, TerminalToken::Function)?;
            Ok((tokens, (return_type, function)))
        },
        |tokens, (return_type, function)| {
            let (tokens, declaration) = function_declaration(tokens)?;
            Ok((
                tokens,
                FunctionExpression {
                    return_type,
                    function,
                    declaration,
                },
            ))
        },
    )
}

fn precedence_error(tokens: TokenList) -> ParseError {
    ParseError::new(
        ParseErrorType::Internal(InternalErrorType::PrecedenceMismatch),
        tokens.start_index(),
    )
}

struct ExpressionRef<'a, 's>(&'a mut Option<Expression<'s>>);
impl<'a, 's> ExpressionRef<'a, 's> {
    fn take(self) -> Expression<'s> {
        self.0.take().unwrap()
    }
}

fn operator<'s>(
    tokens: TokenList<'s>,
    precedence: Precedence,
    left: ExpressionRef<'_, 's>,
) -> ParseResult<'s, Expression<'s>> {
    let left_ref = left.0;
    alt_map(
        property(tokens, precedence, ExpressionRef(left_ref)),
        Expression::Property,
    )
    .or_else(|| {
        alt_map(
            ternary(tokens, precedence, ExpressionRef(left_ref)),
            Expression::Ternary,
        )
    })
    .or_else(|| {
        alt_map(
            binary(tokens, precedence, ExpressionRef(left_ref)),
            Expression::Binary,
        )
    })
    .or_else(|| {
        alt_map(
            index(tokens, precedence, ExpressionRef(left_ref)),
            Expression::Index,
        )
    })
    .or_else(|| {
        alt_map(
            postfix(tokens, precedence, ExpressionRef(left_ref)),
            Expression::Postfix,
        )
    })
    .or_else(|| {
        alt_map(
            call(tokens, precedence, ExpressionRef(left_ref)),
            Expression::Call,
        )
    })
    .or_else(|| {
        alt_map(
            comma(tokens, precedence, ExpressionRef(left_ref)),
            Expression::Comma,
        )
    })
    .unwrap_or_else(|| {
        Err(ParseError::new(
            ParseErrorType::ExpectedOperator,
            tokens.start_index(),
        ))
    })
}

fn property<'s>(
    tokens: TokenList<'s>,
    precedence: Precedence,
    left: ExpressionRef<'_, 's>,
) -> ParseResult<'s, PropertyExpression<'s>> {
    // left associative
    if precedence >= Precedence::Property {
        return Err(precedence_error(tokens));
    }

    definitely(
        tokens,
        ContextType::ExpressionRightHandSide,
        |tokens| terminal(tokens, TerminalToken::Dot),
        |tokens, dot| {
            let (tokens, property) = method_identifier(tokens)?;

            Ok((
                tokens,
                PropertyExpression {
                    base: Box::new(left.take()),
                    dot,
                    property,
                },
            ))
        },
    )
}

fn ternary<'s>(
    tokens: TokenList<'s>,
    precedence: Precedence,
    left: ExpressionRef<'_, 's>,
) -> ParseResult<'s, TernaryExpression<'s>> {
    // right associative
    if precedence > Precedence::Ternary {
        return Err(precedence_error(tokens));
    }

    definitely(
        tokens,
        ContextType::TernaryExpression,
        |tokens| terminal(tokens, TerminalToken::Question),
        |tokens, question| {
            let (tokens, true_value) = expression(tokens, Precedence::None)?;
            let (tokens, separator) = terminal(tokens, TerminalToken::Colon)?;
            let (tokens, false_value) = expression(tokens, Precedence::Ternary)?;

            Ok((
                tokens,
                TernaryExpression {
                    condition: Box::new(left.take()),
                    question,
                    true_value: Box::new(true_value),
                    separator,
                    false_value: Box::new(false_value),
                },
            ))
        },
    )
}

fn binary<'s>(
    tokens: TokenList<'s>,
    precedence: Precedence,
    left: ExpressionRef<'_, 's>,
) -> ParseResult<'s, BinaryExpression<'s>> {
    definitely(
        tokens,
        ContextType::ExpressionRightHandSide,
        |tokens| {
            let (tokens, operator) = binary_operator(tokens)?;

            // left associative
            if precedence >= operator.precedence() {
                Err(precedence_error(tokens))
            } else {
                Ok((tokens, operator))
            }
        },
        |tokens, operator| {
            let (tokens, right) = expression(tokens, operator.precedence())?;
            Ok((
                tokens,
                BinaryExpression {
                    left: Box::new(left.take()),
                    operator,
                    right: Box::new(right),
                },
            ))
        },
    )
}

fn index<'s>(
    tokens: TokenList<'s>,
    precedence: Precedence,
    left: ExpressionRef<'_, 's>,
) -> ParseResult<'s, IndexExpression<'s>> {
    // left associative
    if precedence >= Precedence::Postfix {
        return Err(precedence_error(tokens));
    }

    span(
        tokens,
        ContextType::IndexExpression,
        TerminalToken::OpenSquare,
        TerminalToken::CloseSquare,
        |tokens, open, close| {
            let (tokens, index) = expression(tokens, Precedence::None)?;

            Ok((
                tokens,
                IndexExpression {
                    base: Box::new(left.take()),
                    open,
                    index: Box::new(index),
                    close,
                },
            ))
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
        return Err(precedence_error(tokens));
    }

    let (tokens, operator) = postfix_operator(tokens)?;
    Ok((
        tokens,
        PostfixExpression {
            value: Box::new(left.take()),
            operator,
        },
    ))
}

fn call<'s>(
    tokens: TokenList<'s>,
    precedence: Precedence,
    left: ExpressionRef<'_, 's>,
) -> ParseResult<'s, CallExpression<'s>> {
    // left associative
    if precedence >= Precedence::Postfix {
        return Err(precedence_error(tokens));
    }

    definitely(
        tokens,
        ContextType::CallExpression,
        |tokens| {
            span(
                tokens,
                ContextType::CallExpression,
                TerminalToken::OpenBracket,
                TerminalToken::CloseBracket,
                |tokens, open, close| {
                    let (tokens, arguments) = separated_list_trailing0(
                        tokens,
                        |tokens| expression(tokens, Precedence::Comma),
                        |tokens| terminal(tokens, TerminalToken::Comma),
                    )?;
                    Ok((tokens, (open, arguments, close)))
                },
            )
        },
        |tokens, (open, arguments, close)| {
            // Post-initializer table is only valid if there isn't a newline between the call and
            // it. Otherwise block statements following function calls will be parsed as tables.
            let (tokens, post_initializer) = if tokens.is_newline() {
                (tokens, None)
            } else {
                opt(tokens, table(tokens))?
            };

            Ok((
                tokens,
                CallExpression {
                    function: Box::new(left.take()),
                    open,
                    arguments,
                    close,
                    post_initializer,
                },
            ))
        },
    )
}

fn comma<'s>(
    tokens: TokenList<'s>,
    precedence: Precedence,
    left: ExpressionRef<'_, 's>,
) -> ParseResult<'s, CommaExpression<'s>> {
    // left associative
    if precedence >= Precedence::Comma {
        return Err(precedence_error(tokens));
    }

    definitely(
        tokens,
        ContextType::CommaExpression,
        |tokens| terminal(tokens, TerminalToken::Comma),
        |tokens, first_comma| {
            let (tokens, mut values) = separated_list1(
                tokens,
                |tokens| expression(tokens, Precedence::Comma),
                |tokens| terminal(tokens, TerminalToken::Comma),
            )?;
            values.items.insert(0, (left.take(), first_comma));

            Ok((tokens, CommaExpression { values }))
        },
    )
}
