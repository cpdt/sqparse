use crate::ast::{BinaryOperator, PostfixOperator, PrefixOperator};
use crate::parser::token::{terminal2, terminal3};
use crate::parser::token_list::TokenList;
use crate::parser::{ParseError, ParseErrorType, ParseResult};
use crate::token::{TerminalToken, Token, TokenType};

fn first_terminal(tokens: TokenList) -> Option<(TokenList, &Token, TerminalToken)> {
    if let Some((tokens, item)) = tokens.split_first() {
        if let TokenType::Terminal(terminal) = item.token.ty {
            return Some((tokens, &item.token, terminal));
        }
    }
    None
}

pub fn prefix_operator(tokens: TokenList) -> ParseResult<PrefixOperator> {
    let (remaining_tokens, token, terminal) = first_terminal(tokens).ok_or_else(|| {
        ParseError::new(ParseErrorType::ExpectedPrefixOperator, tokens.start_index())
    })?;

    let prefix = match terminal {
        TerminalToken::Subtract => PrefixOperator::Negate(token),
        TerminalToken::Not => PrefixOperator::LogicalNot(token),
        TerminalToken::BitwiseNot => PrefixOperator::BitwiseNot(token),
        TerminalToken::Typeof => PrefixOperator::Typeof(token),
        TerminalToken::Clone => PrefixOperator::Clone(token),
        TerminalToken::Delete => PrefixOperator::Delete(token),
        TerminalToken::Increment => PrefixOperator::Increment(token),
        TerminalToken::Decrement => PrefixOperator::Decrement(token),
        _ => {
            return Err(ParseError::new(
                ParseErrorType::ExpectedPrefixOperator,
                tokens.start_index(),
            ))
        }
    };

    Ok((remaining_tokens, prefix))
}

pub fn postfix_operator(tokens: TokenList) -> ParseResult<PostfixOperator> {
    let (remaining_tokens, token, terminal) = first_terminal(tokens).ok_or_else(|| {
        ParseError::new(
            ParseErrorType::ExpectedPostfixOperator,
            tokens.start_index(),
        )
    })?;

    let prefix = match terminal {
        TerminalToken::Increment => PostfixOperator::Increment(token),
        TerminalToken::Decrement => PostfixOperator::Decrement(token),
        _ => {
            return Err(ParseError::new(
                ParseErrorType::ExpectedPostfixOperator,
                tokens.start_index(),
            ))
        }
    };

    Ok((remaining_tokens, prefix))
}

pub fn binary_operator(tokens: TokenList) -> ParseResult<BinaryOperator> {
    try_compound_binary_operator(tokens)
        .or_else(|| try_simple_binary_operator(tokens))
        .ok_or_else(|| {
            ParseError::new(ParseErrorType::ExpectedBinaryOperator, tokens.start_index())
        })
}

fn try_compound_binary_operator(tokens: TokenList) -> Option<(TokenList, BinaryOperator)> {
    if let Ok((tokens, (a, b))) = terminal2(tokens, TerminalToken::Less, TerminalToken::Subtract) {
        Some((tokens, BinaryOperator::AssignNewSlot(a, b)))
    } else if let Ok((tokens, (a, b))) = terminal2(tokens, TerminalToken::Less, TerminalToken::Less)
    {
        Some((tokens, BinaryOperator::ShiftLeft(a, b)))
    } else if let Ok((tokens, (a, b, c))) = terminal3(
        tokens,
        TerminalToken::Greater,
        TerminalToken::Greater,
        TerminalToken::Greater,
    ) {
        Some((tokens, BinaryOperator::UnsignedShiftRight(a, b, c)))
    } else if let Ok((tokens, (a, b))) =
        terminal2(tokens, TerminalToken::Greater, TerminalToken::Greater)
    {
        Some((tokens, BinaryOperator::ShiftRight(a, b)))
    } else {
        None
    }
}

fn try_simple_binary_operator(tokens: TokenList) -> Option<(TokenList, BinaryOperator)> {
    let (tokens, token, terminal) = first_terminal(tokens)?;

    let prefix = match terminal {
        TerminalToken::Assign => BinaryOperator::Assign(token),
        TerminalToken::AddEqual => BinaryOperator::AssignAdd(token),
        TerminalToken::SubtractEqual => BinaryOperator::AssignSubtract(token),
        TerminalToken::MultiplyEqual => BinaryOperator::AssignMultiply(token),
        TerminalToken::DivideEqual => BinaryOperator::AssignDivide(token),
        TerminalToken::ModuloEqual => BinaryOperator::AssignModulo(token),
        TerminalToken::Add => BinaryOperator::Add(token),
        TerminalToken::Subtract => BinaryOperator::Subtract(token),
        TerminalToken::Multiply => BinaryOperator::Multiply(token),
        TerminalToken::Divide => BinaryOperator::Divide(token),
        TerminalToken::Modulo => BinaryOperator::Modulo(token),
        TerminalToken::Equal => BinaryOperator::Equal(token),
        TerminalToken::NotEqual => BinaryOperator::NotEqual(token),
        TerminalToken::Less => BinaryOperator::Less(token),
        TerminalToken::LessEqual => BinaryOperator::LessEqual(token),
        TerminalToken::Greater => BinaryOperator::Greater(token),
        TerminalToken::GreaterEqual => BinaryOperator::GreaterEqual(token),
        TerminalToken::ThreeWay => BinaryOperator::ThreeWay(token),
        TerminalToken::LogicalAnd => BinaryOperator::LogicalAnd(token),
        TerminalToken::LogicalOr => BinaryOperator::LogicalOr(token),
        TerminalToken::BitwiseAnd => BinaryOperator::BitwiseAnd(token),
        TerminalToken::BitwiseOr => BinaryOperator::BitwiseOr(token),
        TerminalToken::BitwiseXor => BinaryOperator::BitwiseXor(token),
        TerminalToken::In => BinaryOperator::In(token),
        TerminalToken::Instanceof => BinaryOperator::Instanceof(token),
        _ => return None,
    };

    Some((tokens, prefix))
}
