use crate::ast::{
    ArrayExpression, ArrayValue, BinaryOperatorExpression, BinaryOperatorType, CallExpression,
    ClassExpression, CommaExpression, DelegateExpression, ExpectExpression, Expression,
    FunctionExpression, Identifier, IndexExpression, ParensExpression, PostfixOperatorExpression,
    PostfixOperatorType, PrefixOperatorExpression, PrefixOperatorType, PropertyExpression,
    RootVarExpression, SeparatedList0, TableExpression, TableSlot, TableSlotType,
    TernaryOperatorExpression, VectorExpression,
};
use crate::parser::shared::{
    adjacent_2, adjacent_3, class_declaration, function_declaration, identifier, is_end_of_line,
    literal, many_separated_till, not_line_ending, perfect, separated_list0, separated_list1,
    terminal,
};
use crate::parser::ty::ty;
use crate::parser::TokenList;
use crate::token::TerminalToken;
use crate::{IResult, Token};
use nom::branch::alt;
use nom::combinator::{consumed, map, opt};
use nom::multi::{fold_many0, many0};
use nom::sequence::{pair, tuple};

pub fn expression(i: TokenList) -> IResult<TokenList, Expression> {
    comma_expression(i)
}

pub fn non_comma_expression(i: TokenList) -> IResult<TokenList, Expression> {
    assignment_expression(i)
}

fn parens_expression(i: TokenList) -> IResult<TokenList, ParensExpression> {
    map(
        pair(
            terminal(TerminalToken::OpenBracket),
            perfect(
                "parens expression",
                pair(expression, terminal(TerminalToken::CloseBracket)),
            ),
        ),
        |(open_token, (value, close_token))| ParensExpression {
            value: Box::new(value),

            open_token,
            close_token,
        },
    )(i)
}

fn vector_expression(i: TokenList) -> IResult<TokenList, VectorExpression> {
    map(
        pair(
            terminal(TerminalToken::Less),
            perfect(
                "vector expression",
                tuple((
                    non_comma_expression,
                    terminal(TerminalToken::Comma),
                    non_comma_expression,
                    terminal(TerminalToken::Comma),
                    math_binary_operator_expression,
                    terminal(TerminalToken::Greater),
                )),
            ),
        ),
        |(
            open_token,
            (x_val, first_separator_token, y_val, second_separator_token, z_val, close_token),
        )| {
            VectorExpression {
                x_val: Box::new(x_val),
                y_val: Box::new(y_val),
                z_val: Box::new(z_val),

                open_token,
                first_separator_token,
                second_separator_token,
                close_token,
            }
        },
    )(i)
}

fn root_var_expression(i: TokenList) -> IResult<TokenList, RootVarExpression> {
    map(
        pair(
            terminal(TerminalToken::Namespace),
            perfect("root var", identifier),
        ),
        |(root_token, name)| RootVarExpression { name, root_token },
    )(i)
}

fn table_slot_type(i: TokenList) -> IResult<TokenList, TableSlotType> {
    let property_slot = map(
        pair(
            identifier,
            perfect(
                "table property",
                pair(terminal(TerminalToken::Assign), non_comma_expression),
            ),
        ),
        |(name, (separator_token, value))| TableSlotType::Property {
            name,
            value: Box::new(value),

            separator_token,
        },
    );
    let computed_property_slot = map(
        pair(
            terminal(TerminalToken::OpenSquare),
            perfect(
                "table property",
                tuple((
                    non_comma_expression,
                    terminal(TerminalToken::CloseSquare),
                    terminal(TerminalToken::Assign),
                    non_comma_expression,
                )),
            ),
        ),
        |(open_name_token, (name, close_name_token, separator_token, value))| {
            TableSlotType::ComputedProperty {
                name: Box::new(name),
                value: Box::new(value),

                open_name_token,
                close_name_token,
                separator_token,
            }
        },
    );
    let json_property_slot = map(
        pair(
            literal,
            perfect(
                "table property",
                pair(terminal(TerminalToken::Colon), non_comma_expression),
            ),
        ),
        |(name, (separator_token, value))| TableSlotType::JsonProperty {
            name,
            value: Box::new(value),

            separator_token,
        },
    );
    let function_slot = alt((
        map(
            pair(
                terminal(TerminalToken::Function),
                perfect(
                    "function declaration",
                    pair(identifier, function_declaration),
                ),
            ),
            |(function_token, (name, function))| TableSlotType::Function {
                return_type: None,
                name,
                function: Box::new(function),

                function_token,
            },
        ),
        map(
            tuple((
                ty(false),
                terminal(TerminalToken::Function),
                perfect(
                    "function declaration",
                    pair(identifier, function_declaration),
                ),
            )),
            |(return_type, function_token, (name, function))| TableSlotType::Function {
                return_type: Some(return_type),
                name,
                function: Box::new(function),

                function_token,
            },
        ),
    ));

    alt((
        property_slot,
        computed_property_slot,
        json_property_slot,
        function_slot,
    ))(i)
}

pub fn table_expression_delimited<'s>(
    open: TerminalToken,
    close: TerminalToken,
) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, TableExpression<'s>> {
    map(
        pair(
            terminal(open),
            perfect(
                "table",
                many_separated_till(
                    table_slot_type,
                    terminal(TerminalToken::Comma),
                    pair(opt(terminal(TerminalToken::Ellipsis)), terminal(close)),
                    |ty, separator_token| TableSlot {
                        ty,
                        separator_token,
                    },
                ),
            ),
        ),
        |(open_token, (slots, (spread_token, close_token)))| TableExpression {
            slots,

            open_token,
            spread_token,
            close_token,
        },
    )
}

fn table_expression(i: TokenList) -> IResult<TokenList, TableExpression> {
    table_expression_delimited(TerminalToken::OpenBrace, TerminalToken::CloseBrace)(i)
}

fn class_expression(i: TokenList) -> IResult<TokenList, ClassExpression> {
    map(
        pair(terminal(TerminalToken::Class), class_declaration),
        |(class_token, class)| ClassExpression { class, class_token },
    )(i)
}

fn array_expression(i: TokenList) -> IResult<TokenList, ArrayExpression> {
    map(
        pair(
            terminal(TerminalToken::OpenSquare),
            perfect(
                "array",
                many_separated_till(
                    non_comma_expression,
                    terminal(TerminalToken::Comma),
                    pair(
                        opt(terminal(TerminalToken::Ellipsis)),
                        terminal(TerminalToken::CloseSquare),
                    ),
                    |value, separator_token| ArrayValue {
                        value: Box::new(value),
                        separator_token,
                    },
                ),
            ),
        ),
        |(open_token, (values, (spread_token, close_token)))| ArrayExpression {
            values,

            open_token,
            spread_token,
            close_token,
        },
    )(i)
}

fn expect_expression(i: TokenList) -> IResult<TokenList, ExpectExpression> {
    map(
        pair(
            terminal(TerminalToken::Expect),
            perfect(
                "expect",
                tuple((
                    ty(true),
                    terminal(TerminalToken::OpenBracket),
                    expression,
                    terminal(TerminalToken::CloseBracket),
                )),
            ),
        ),
        |(expect_token, (expected_type, open_value_token, value, close_value_token))| {
            ExpectExpression {
                expected_type,
                value: Box::new(value),

                expect_token,
                open_value_token,
                close_value_token,
            }
        },
    )(i)
}

fn base_expression(i: TokenList) -> IResult<TokenList, Expression> {
    alt((
        map(
            pair(
                terminal(TerminalToken::Function),
                perfect("inline function", function_declaration),
            ),
            |(function_token, declaration)| {
                Expression::Function(FunctionExpression {
                    return_type: None,
                    declaration,

                    function_token,
                })
            },
        ),
        map(
            tuple((
                ty(false),
                terminal(TerminalToken::Function),
                perfect("inline function", function_declaration),
            )),
            |(return_type, function_token, declaration)| {
                Expression::Function(FunctionExpression {
                    return_type: Some(return_type),
                    declaration,

                    function_token,
                })
            },
        ),
        map(parens_expression, Expression::Parens),
        map(literal, Expression::Literal),
        map(vector_expression, Expression::Vector),
        map(identifier, Expression::Var),
        map(root_var_expression, Expression::RootVar),
        map(table_expression, Expression::Table),
        map(class_expression, Expression::Class),
        map(array_expression, Expression::Array),
        map(expect_expression, Expression::Expect),
    ))(i)
}

// Expressions where we know the right params before the left expression
enum AlmostExpression<'s> {
    Call {
        arguments: SeparatedList0<'s, Expression<'s>>,
        post_initializer: Option<TableExpression<'s>>,

        open_arguments_token: &'s Token<'s>,
        trailing_separator_token: Option<&'s Token<'s>>,
        close_arguments_token: &'s Token<'s>,
    },
    Index {
        index: Box<Expression<'s>>,

        open_index_token: &'s Token<'s>,
        close_index_token: &'s Token<'s>,
    },
    Property {
        property: Identifier<'s>,

        separator_token: &'s Token<'s>,
    },
}

impl<'s> AlmostExpression<'s> {
    fn into_expr(self, base: Expression<'s>) -> Expression<'s> {
        match self {
            AlmostExpression::Call {
                arguments,
                post_initializer,
                open_arguments_token,
                trailing_separator_token,
                close_arguments_token,
            } => Expression::Call(CallExpression {
                function: Box::new(base),
                arguments,
                post_initializer,

                open_arguments_token,
                trailing_separator_token,
                close_arguments_token,
            }),
            AlmostExpression::Index {
                index,
                open_index_token,
                close_index_token,
            } => Expression::Index(IndexExpression {
                base: Box::new(base),
                index,

                open_index_token,
                close_index_token,
            }),
            AlmostExpression::Property {
                property,
                separator_token,
            } => Expression::Property(PropertyExpression {
                base: Box::new(base),
                property,

                separator_token,
            }),
        }
    }
}

fn call_expression(i: TokenList) -> IResult<TokenList, AlmostExpression> {
    let (
        i,
        (
            consumed,
            (open_arguments_token, (arguments, trailing_separator_token, close_arguments_token)),
        ),
    ) = consumed(pair(
        terminal(TerminalToken::OpenBracket),
        perfect(
            "call",
            tuple((
                separated_list0(TerminalToken::Comma, non_comma_expression),
                opt(terminal(TerminalToken::Comma)),
                terminal(TerminalToken::CloseBracket),
            )),
        ),
    ))(i)?;

    // A post initializer must follow on this line
    let (i, post_initializer) = if consumed.end_of_line() {
        (i, None)
    } else {
        opt(table_expression)(i)?
    };

    Ok((
        i,
        AlmostExpression::Call {
            arguments,
            post_initializer,

            open_arguments_token,
            trailing_separator_token,
            close_arguments_token,
        },
    ))
}

fn index_expression(i: TokenList) -> IResult<TokenList, AlmostExpression> {
    map(
        pair(
            terminal(TerminalToken::OpenSquare),
            perfect(
                "index",
                pair(expression, terminal(TerminalToken::CloseSquare)),
            ),
        ),
        |(open_index_token, (index, close_index_token))| AlmostExpression::Index {
            index: Box::new(index),

            open_index_token,
            close_index_token,
        },
    )(i)
}

fn property_expression(i: TokenList) -> IResult<TokenList, AlmostExpression> {
    map(
        tuple((
            terminal(TerminalToken::Dot),
            perfect("property", identifier),
        )),
        |(separator_token, property)| AlmostExpression::Property {
            property,

            separator_token,
        },
    )(i)
}

fn access_expression(i: TokenList) -> IResult<TokenList, Expression> {
    let (i, base) = base_expression(i)?;

    fold_many0(
        alt((call_expression, index_expression, property_expression)),
        move || base.clone(),
        |base, almost_expr| almost_expr.into_expr(base),
    )(i)
}

fn delegate_expression(i: TokenList) -> IResult<TokenList, Expression> {
    let (i, delegates) = many0(pair(
        terminal(TerminalToken::Delegate),
        perfect(
            "delegate expression",
            pair(expression, terminal(TerminalToken::Colon)),
        ),
    ))(i)?;
    let (i, last_value) = access_expression(i)?;

    let value = delegates.into_iter().rfold(
        last_value,
        |delegate, (delegate_token, (base, separator_token))| {
            Expression::Delegate(DelegateExpression {
                base: Box::new(base),
                delegate: Box::new(delegate),

                delegate_token,
                separator_token,
            })
        },
    );
    Ok((i, value))
}

fn postfix_operator_expression(i: TokenList) -> IResult<TokenList, Expression> {
    let postfix_type = alt((
        map(terminal(TerminalToken::Increment), |t| {
            (t, PostfixOperatorType::Increment)
        }),
        map(terminal(TerminalToken::Decrement), |t| {
            (t, PostfixOperatorType::Decrement)
        }),
    ));
    let (i, (eol, base)) = is_end_of_line(delegate_expression)(i)?;

    // We can only parse postfix operators if this isn't the end of the line
    if eol {
        Ok((i, base))
    } else {
        fold_many0(
            postfix_type,
            move || base.clone(),
            |value, (operator_token, ty)| {
                Expression::PostfixOperator(PostfixOperatorExpression {
                    ty,
                    value: Box::new(value),

                    operator_token,
                })
            },
        )(i)
    }
}

fn prefix_operator_expression(i: TokenList) -> IResult<TokenList, Expression> {
    let prefix_type = alt((
        map(terminal(TerminalToken::Subtract), |t| {
            (t, PrefixOperatorType::Negate)
        }),
        map(terminal(TerminalToken::Not), |t| {
            (t, PrefixOperatorType::LogicalNot)
        }),
        map(terminal(TerminalToken::BitwiseNot), |t| {
            (t, PrefixOperatorType::BitwiseNot)
        }),
        map(terminal(TerminalToken::Typeof), |t| {
            (t, PrefixOperatorType::Typeof)
        }),
        map(terminal(TerminalToken::Clone), |t| {
            (t, PrefixOperatorType::Clone)
        }),
        map(terminal(TerminalToken::Delete), |t| {
            (t, PrefixOperatorType::Delete)
        }),
        map(terminal(TerminalToken::Increment), |t| {
            (t, PrefixOperatorType::Increment)
        }),
        map(terminal(TerminalToken::Decrement), |t| {
            (t, PrefixOperatorType::Decrement)
        }),
    ));
    let (i, prefixes) = many0(not_line_ending(prefix_type))(i)?;
    let (i, value) = postfix_operator_expression(i)?;

    let value = prefixes
        .into_iter()
        .rfold(value, |value, (operator_token, ty)| {
            Expression::PrefixOperator(PrefixOperatorExpression {
                ty,
                value: Box::new(value),

                operator_token,
            })
        });
    Ok((i, value))
}

fn binary_expression<'s>(
    mut inner_precedence: impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, Expression<'s>>,
    mut operator: impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, BinaryOperatorType>,
) -> impl FnMut(TokenList<'s>) -> IResult<TokenList<'s>, Expression<'s>> {
    move |i| {
        let (i, left_value) = inner_precedence(i)?;

        fold_many0(
            pair(|i| operator(i), perfect("RHS", |i| inner_precedence(i))),
            move || left_value.clone(),
            |left_value, (ty, right_value)| {
                Expression::BinaryOperator(BinaryOperatorExpression {
                    ty,
                    left_value: Box::new(left_value),
                    right_value: Box::new(right_value),
                })
            },
        )(i)
    }
}

fn math_binary_operator_expression(i: TokenList) -> IResult<TokenList, Expression> {
    let parser = binary_expression(
        prefix_operator_expression,
        alt((
            map(terminal(TerminalToken::Divide), BinaryOperatorType::Divide),
            map(
                terminal(TerminalToken::Multiply),
                BinaryOperatorType::Multiply,
            ),
            map(terminal(TerminalToken::Modulo), BinaryOperatorType::Modulo),
        )),
    );
    let parser = binary_expression(
        parser,
        alt((
            map(terminal(TerminalToken::Add), BinaryOperatorType::Add),
            map(
                terminal(TerminalToken::Subtract),
                BinaryOperatorType::Subtract,
            ),
        )),
    );
    let mut parser = binary_expression(
        parser,
        alt((
            map(
                adjacent_2(TerminalToken::Less, TerminalToken::Less),
                |(t1, t2)| BinaryOperatorType::ShiftLeft(t1, t2),
            ),
            map(
                adjacent_3(
                    TerminalToken::Greater,
                    TerminalToken::Greater,
                    TerminalToken::Greater,
                ),
                |(t1, t2, t3)| BinaryOperatorType::UnsignedShiftRight(t1, t2, t3),
            ),
            map(
                adjacent_2(TerminalToken::Greater, TerminalToken::Greater),
                |(t1, t2)| BinaryOperatorType::ShiftRight(t1, t2),
            ),
        )),
    );

    parser(i)
}

fn comparison_operator_expression(i: TokenList) -> IResult<TokenList, Expression> {
    let parser = binary_expression(
        math_binary_operator_expression,
        alt((
            map(terminal(TerminalToken::Less), BinaryOperatorType::Less),
            map(
                terminal(TerminalToken::LessEqual),
                BinaryOperatorType::LessEqual,
            ),
            map(
                terminal(TerminalToken::Greater),
                BinaryOperatorType::Greater,
            ),
            map(
                terminal(TerminalToken::GreaterEqual),
                BinaryOperatorType::GreaterEqual,
            ),
            map(
                terminal(TerminalToken::ThreeWay),
                BinaryOperatorType::ThreeWay,
            ),
        )),
    );
    let parser = binary_expression(
        parser,
        alt((
            map(terminal(TerminalToken::Equal), BinaryOperatorType::Equal),
            map(
                terminal(TerminalToken::NotEqual),
                BinaryOperatorType::NotEqual,
            ),
        )),
    );
    let parser = binary_expression(
        parser,
        map(
            terminal(TerminalToken::BitwiseAnd),
            BinaryOperatorType::BitwiseAnd,
        ),
    );
    let parser = binary_expression(
        parser,
        map(
            terminal(TerminalToken::BitwiseXor),
            BinaryOperatorType::BitwiseXor,
        ),
    );
    let parser = binary_expression(
        parser,
        map(
            terminal(TerminalToken::BitwiseOr),
            BinaryOperatorType::BitwiseOr,
        ),
    );
    let parser = binary_expression(
        parser,
        alt((
            map(
                terminal(TerminalToken::LogicalAnd),
                BinaryOperatorType::LogicalAnd,
            ),
            map(terminal(TerminalToken::In), BinaryOperatorType::In),
            map(
                terminal(TerminalToken::Instanceof),
                BinaryOperatorType::Instanceof,
            ),
        )),
    );
    let mut parser = binary_expression(
        parser,
        map(
            terminal(TerminalToken::LogicalOr),
            BinaryOperatorType::LogicalOr,
        ),
    );

    parser(i)
}

fn ternary_operator_expression(i: TokenList) -> IResult<TokenList, Expression> {
    let (i, ternaries) = many0(tuple((
        comparison_operator_expression,
        terminal(TerminalToken::Question),
        perfect("ternary truthy", comparison_operator_expression),
        terminal(TerminalToken::Colon),
    )))(i)?;
    let (i, false_value) = comparison_operator_expression(i)?;

    let value = ternaries.into_iter().rfold(
        false_value,
        |false_value, (condition, question_token, true_value, colon_token)| {
            Expression::TernaryOperator(TernaryOperatorExpression {
                condition: Box::new(condition),
                true_value: Box::new(true_value),
                false_value: Box::new(false_value),

                question_token,
                colon_token,
            })
        },
    );
    Ok((i, value))
}

fn assignment_expression(i: TokenList) -> IResult<TokenList, Expression> {
    binary_expression(
        ternary_operator_expression,
        alt((
            map(terminal(TerminalToken::Assign), BinaryOperatorType::Assign),
            map(
                adjacent_2(TerminalToken::Less, TerminalToken::Subtract),
                |(t1, t2)| BinaryOperatorType::AssignNewSlot(t1, t2),
            ),
            map(
                terminal(TerminalToken::AddEqual),
                BinaryOperatorType::AssignAdd,
            ),
            map(
                terminal(TerminalToken::SubtractEqual),
                BinaryOperatorType::AssignSubtract,
            ),
            map(
                terminal(TerminalToken::MultiplyEqual),
                BinaryOperatorType::AssignMultiply,
            ),
            map(
                terminal(TerminalToken::DivideEqual),
                BinaryOperatorType::AssignDivide,
            ),
            map(
                terminal(TerminalToken::ModuloEqual),
                BinaryOperatorType::AssignModulo,
            ),
        )),
    )(i)
}

fn comma_expression(i: TokenList) -> IResult<TokenList, Expression> {
    let (i, values) = separated_list1(TerminalToken::Comma, non_comma_expression)(i)?;
    let expression = if values.items.is_empty() {
        *values.last_item
    } else {
        Expression::Comma(CommaExpression { values })
    };
    Ok((i, expression))
}
