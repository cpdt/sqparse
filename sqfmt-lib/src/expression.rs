use sqparse::ast::{ArrayExpression, ClassExpression, Expression, IndexExpression, LiteralExpression, ParensExpression, PropertyExpression, RootVarExpression, TableExpression, TernaryOperatorExpression};
use crate::combinators::{alt, cond, cond_or, empty_line, format, indented, iter, opt, opt_or, pair, single_line, space, tag, tuple};
use crate::shared::{identifier, optional_separator, token_or_tag};
use crate::token::{discard_token, token};
use crate::writer::Writer;

pub fn expression<'s>(expr: &'s Expression<'s>, parent_precedence: u32) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    move |mut i| {
        if let Expression::Parens(parens_expr) = expr {
            let mut parens_expressions = Vec::new();
            let mut inner_parens_expression = parens_expr;
            loop {
                match inner_parens_expression.value.as_ref() {
                    Expression::Parens(inner_parens) => {
                        parens_expressions.push(inner_parens_expression);
                        inner_parens_expression = inner_parens;
                    },
                    _ => break,
                }
            }

            for discard_expression in &parens_expressions {
                i = discard_token(&discard_expression.open_token)(i)?;
            }

            let needs_parens = inner_parens_expression.value.precedence() > parent_precedence;
            if needs_parens {
                i = parens_expression(inner_parens_expression)(i)?;
            } else {
                i = tuple((
                    discard_token(&inner_parens_expression.open_token),
                    expression(inner_parens_expression.value.as_ref(), parent_precedence),
                    discard_token(&inner_parens_expression.close_token),
                ))(i)?;
            }

            for discard_expression in parens_expressions.iter().rev() {
                i = discard_token(&discard_expression.close_token)(i)?;
            }

            return Some(i);
        }

        let needs_parens = expr.precedence() > parent_precedence;
        if needs_parens {
            // todo: merge this with parens_expression?
            alt(
                single_line(tuple((
                    tag("("),
                    format(|f| f.spaces_in_expr_brackets, space),
                    expression_without_parens(expr, u32::MAX),
                    format(|f| f.spaces_in_expr_brackets, space),
                    tag(")")
                ))),
                tuple((
                    tag("("),
                    indented(pair(empty_line, expression_without_parens(expr, u32::MAX))),
                    empty_line,
                    tag(")")
                ))
            )(i)
        } else {
            expression_without_parens(expr, u32::MAX)(i)
        }
    }
}

fn expression_without_parens<'s>(expr: &'s Expression<'s>, parent_precedence: u32) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    let precedence = expr.precedence();
    move |i| {
        match expr {
            Expression::Parens(_) => unreachable!(),
            Expression::Literal(expr) => literal_expression(expr)(i),
            Expression::Var(expr) => identifier(expr)(i),
            Expression::RootVar(expr) => root_var_expression(expr)(i),
            Expression::Table(expr) => table_expression(expr)(i),
            Expression::Class(expr) => class_expression(expr)(i),
            Expression::Array(expr) => array_expression(expr)(i),
            Expression::Index(expr) => index_expression(expr, precedence)(i),
            Expression::Property(expr) => property_expression(expr, precedence)(i),
            Expression::TernaryOperator(expr) => ternary_operator_expression(expr, precedence)(i),
            _ => todo!(),

            /*Expression::Index(_) => {}
            Expression::Property(_) => {}
            Expression::TernaryOperator(_) => {}
            Expression::BinaryOperator(_) => {}
            Expression::PostfixOperator(_) => {}
            Expression::PrefixOperator(_) => {}
            Expression::Comma(_) => {}
            Expression::Call(_) => {}
            Expression::Function(_) => {}
            Expression::Delegate(_) => {}
            Expression::Vector(_) => {}
            Expression::Expect(_) => {}*/
        }
    }
}

fn parens_expression<'s>(expr: &'s ParensExpression<'s>) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    alt(
        single_line(tuple((
            token(&expr.open_token),
            format(|f| f.spaces_in_expr_brackets, space),
            expression(&expr.value, u32::MAX),
            format(|f| f.spaces_in_expr_brackets, space),
            token(&expr.close_token)
        ))),
        tuple((
            token(&expr.open_token),
            indented(pair(empty_line, expression(&expr.value, u32::MAX))),
            empty_line,
            token(&expr.close_token),
        )),
    )
}

fn literal_expression<'s>(expr: &'s LiteralExpression<'s>) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    token(expr.token)
}

fn root_var_expression<'s>(expr: &'s RootVarExpression<'s>) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    pair(token(expr.root_token), identifier(&expr.name))
}

fn table_expression<'s>(expr: &'s TableExpression<'s>) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    move |i| todo!()
}

fn class_expression<'s>(expr: &'s ClassExpression<'s>) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    move |i| todo!()
}

fn array_expression<'s>(expr: &'s ArrayExpression<'s>) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    alt(
        single_line(array_expression_single_line(expr)),
        array_expression_multi_line(expr),
    )
}

fn array_expression_single_line<'s>(expr: &'s ArrayExpression<'s>) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    move |i| {
        let last_needs_trailing = expr.spread_token.is_some() || i.format().array_singleline_trailing_commas;

        tuple((
            token(&expr.open_token),
            format(|f| f.array_spaces, space),

            opt(expr.values.split_last(), |(last_value, first_values)| tuple((
                iter(first_values.iter().map(|value| tuple((
                    expression(value.value.as_ref(), Expression::NOT_COMMA_PRECEDENCE),
                    token_or_tag(value.separator_token, ","),
                    space,
                )))),

                expression(last_value.value.as_ref(), Expression::NOT_COMMA_PRECEDENCE),
                optional_separator(last_needs_trailing, last_value.separator_token, ","),

                cond(expr.spread_token.is_some(), space),
            ))),
            opt(expr.spread_token, token),

            format(|f| f.array_spaces, space),
            token(&expr.close_token),
        ))(i)
    }
}

fn array_expression_multi_line<'s>(expr: &'s ArrayExpression<'s>) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    move |i| {
        let has_commas = i.format().array_multiline_commas || i.format().array_multiline_trailing_commas;
        let last_has_comma = expr.spread_token.is_some() || i.format().array_multiline_trailing_commas;

        tuple((
            token(&expr.open_token),
            indented(tuple((
                opt(expr.values.split_last(), |(last_value, first_values)| tuple((
                    iter(first_values.iter().map(|value| tuple((
                        empty_line,
                        expression(value.value.as_ref(), Expression::NOT_COMMA_PRECEDENCE),
                        optional_separator(has_commas, value.separator_token, ","),
                    )))),

                    empty_line,
                    expression(last_value.value.as_ref(), Expression::NOT_COMMA_PRECEDENCE),
                    optional_separator(last_has_comma, last_value.separator_token, ","),
                ))),
                opt(expr.spread_token, |t| pair(empty_line, token(t))),
            ))),
            empty_line,
            token(&expr.close_token),
        ))(i)
    }
}

fn index_expression<'s>(expr: &'s IndexExpression<'s>, precedence: u32) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    // todo: handle a long base expression that goes over multiple lines
    pair(
        expression(&expr.base, precedence),
        alt(
            single_line(tuple((token(expr.open_index_token), expression(&expr.index, Expression::NO_PRECEDENCE), token(expr.close_index_token)))),
            tuple((
                token(expr.open_index_token),
                indented(pair(empty_line, expression(&expr.index, Expression::NO_PRECEDENCE))),
                empty_line,
                token(expr.close_index_token),
            ))
        )
    )
}

fn property_expression<'s>(expr: &'s PropertyExpression<'s>, precedence: u32) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    alt(
        single_line(tuple((
            expression(&expr.base, precedence),
            token(expr.separator_token),
            identifier(&expr.property),
        ))),
        pair(
            expression(&expr.base, precedence),
            indented(tuple((
                empty_line,
                token(expr.separator_token),
                identifier(&expr.property),
            )))
        )
    )
}

fn ternary_operator_expression<'s>(expr: &'s TernaryOperatorExpression<'s>, precedence: u32) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    alt(
        single_line(tuple((
            expression(&expr.condition, precedence),
            space,
            token(&expr.question_token),
            space,
            expression(&expr.true_value, precedence),
            space,
            token(&expr.colon_token),
            space,
            expression(&expr.false_value, Expression::NO_PRECEDENCE),
        ))),
        pair(
            expression(&expr.condition, precedence),
            indented(tuple((
                empty_line,
                token(&expr.question_token),
                space,
                expression(&expr.true_value, precedence),
                empty_line,
                token(&expr.colon_token),
                space,
                expression(&expr.false_value, Expression::NO_PRECEDENCE),
            ))),
        ),
    )
}

#[cfg(test)]
mod tests {
    use sqparse::ast::{ArrayExpression, ArrayValue, Expression, Identifier, IndexExpression, LiteralExpression, PropertyExpression, RootVarExpression};
    use sqparse::ast::SwitchCaseCondition::Expr;
    use sqparse::token::{LiteralBase, LiteralToken, StringToken, TerminalToken, TokenType};
    use crate::config::Format;
    use crate::expression::{array_expression, index_expression, literal_expression, property_expression, root_var_expression, ternary_operator_expression};
    use crate::test_utils::{expr, mock_format, mock_token, test_write, test_write_columns, test_write_format, tokens};

    #[test]
    fn test_literal_expression() {
        let lit = LiteralToken::Char("a");
        let t = mock_token(TokenType::Literal(lit));
        let e = LiteralExpression { literal: lit, token: &t };
        let val = test_write(literal_expression(&e));

        assert_eq!(val, "'a'");
    }

    #[test]
    fn test_root_var_expression() {
        let name_token = mock_token(TokenType::Identifier("hello"));
        let name = Identifier { value: "hello", token: &name_token };
        let root_token = mock_token(TokenType::Terminal(TerminalToken::Namespace));
        let e = RootVarExpression { name, root_token: &root_token };
        let val = test_write(root_var_expression(&e));

        assert_eq!(val, "::hello");
    }

    #[test]
    fn test_empty_array_expression() {
        let open_token = mock_token(TokenType::Terminal(TerminalToken::OpenSquare));
        let close_token = mock_token(TokenType::Terminal(TerminalToken::CloseSquare));
        let e = ArrayExpression {
            values: Vec::new(),
            open_token: &open_token,
            spread_token: None,
            close_token: &close_token,
        };
        let val = test_write(array_expression(&e));
        assert_eq!(val, "[]");

        let val = test_write_format(Format {
            array_spaces: true,
            ..mock_format()
        }, array_expression(&e));
        assert_eq!(val, "[ ]");
    }

    #[test]
    fn test_array_expression() {
        let t = tokens(r#"["hello", "there", 1.2345]"#);
        let e = match expr(&t) {
            Expression::Array(e) => e,
            _ => unreachable!()
        };

        let val = test_write_columns(80, array_expression(&e));
        assert_eq!(val, r#"["hello", "there", 1.2345]"#);

        let val = test_write_columns(20, array_expression(&e));
        assert_eq!(val, r#"
[
    "hello"
    "there"
    1.2345
]"#.trim_start());

        let val = test_write_format(Format {
            column_limit: 80,
            array_spaces: true,
            array_singleline_trailing_commas: true,
            ..mock_format()
        }, array_expression(&e));
        assert_eq!(val, r#"[ "hello", "there", 1.2345, ]"#);

        let val = test_write_format(Format {
            column_limit: 20,
            array_spaces: true,
            array_multiline_trailing_commas: true,
            ..mock_format()
        }, array_expression(&e));
        assert_eq!(val, r#"
[
    "hello",
    "there",
    1.2345,
]"#.trim_start());
    }

    #[test]
    fn test_array_spread_expression() {
        let t = tokens(r#"["general", ...]"#);
        let e = match expr(&t) {
            Expression::Array(e) => e,
            _ => unreachable!()
        };
        let val = test_write(array_expression(&e));
        assert_eq!(val, r#"["general", ...]"#);

        let val = test_write_columns(10, array_expression(&e));
        assert_eq!(val, r#"
[
    "general",
    ...
]"#.trim_start());
    }

    #[test]
    fn test_index_expression() {
        let t = tokens("some_variable_with_a_long_name[5]");
        let e = match expr(&t) {
            Expression::Index(e) => e,
            _ => unreachable!()
        };
        let val = test_write_columns(80, index_expression(&e, Expression::NO_PRECEDENCE));
        assert_eq!(val, "some_variable_with_a_long_name[5]");

        let val = test_write(index_expression(&e, Expression::NO_PRECEDENCE));
        assert_eq!(val, r#"
some_variable_with_a_long_name[
    5
]"#.trim_start());
    }

    #[test]
    fn test_chain_index_expression() {
        let t = tokens("some_variable_with_a_long_name[5][5]");
        let e = match expr(&t) {
            Expression::Index(e) => e,
            _ => unreachable!()
        };
        let val = test_write_columns(80, index_expression(&e, Expression::NO_PRECEDENCE));
        assert_eq!(val, "some_variable_with_a_long_name[5][5]");

        let val = test_write(index_expression(&e, Expression::NO_PRECEDENCE));
        assert_eq!(val, r#"
some_variable_with_a_long_name[
    5
][5]"#.trim_start());
    }

    #[test]
    fn test_property_expression() {
        let t = tokens("some_variable_with_a_long_name.some_property");
        let e = match expr(&t) {
            Expression::Property(e) => e,
            _ => unreachable!()
        };
        let val = test_write_columns(80, property_expression(&e, Expression::NO_PRECEDENCE));
        assert_eq!(val, "some_variable_with_a_long_name.some_property");

        let val = test_write(property_expression(&e, Expression::NO_PRECEDENCE));
        assert_eq!(val, r#"
some_variable_with_a_long_name
    .some_property"#.trim_start());
    }

    #[test]
    fn test_chain_property_expression() {
        let t = tokens("some_variable_with_a_long_name.some_property.some_property");
        let e = match expr(&t) {
            Expression::Property(e) => e,
            _ => unreachable!()
        };
        let val = test_write_columns(80, property_expression(&e, Expression::NO_PRECEDENCE));
        assert_eq!(val, "some_variable_with_a_long_name.some_property.some_property");

        let val = test_write(property_expression(&e, Expression::NO_PRECEDENCE));
        assert_eq!(val, r#"
some_variable_with_a_long_name
    .some_property
    .some_property"#.trim_start());
    }

    #[test]
    fn test_ternary_operator_expression() {
        let t = tokens("some_long_condition_name ? truthy_value : falsy_value");
        let e = match expr(&t) {
            Expression::TernaryOperator(e) => e,
            _ => unreachable!()
        };
        let val = test_write_columns(80, ternary_operator_expression(&e, Expression::NO_PRECEDENCE));
        assert_eq!(val, "some_long_condition_name ? truthy_value : falsy_value");

        let val = test_write(ternary_operator_expression(&e, Expression::NO_PRECEDENCE));
        assert_eq!(val, r#"
some_long_condition_name
    ? truthy_value
    : falsy_value"#.trim_start());
    }

    #[test]
    fn test_chain_ternary_operator_expression() {
        let t = tokens("some_long_condition_name ? truthy_value : second_condition_name ? second_truthy_value : falsy_value");
        let e = match expr(&t) {
            Expression::TernaryOperator(e) => e,
            _ => unreachable!()
        };
        let val = test_write(ternary_operator_expression(&e, Expression::NO_PRECEDENCE));
        assert_eq!(val, r#"
some_long_condition_name
    ? truthy_value
    : second_condition_name
        ? second_truthy_value
        : falsy_value"#.trim_start());
    }
}
