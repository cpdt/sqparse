use crate::ast::{BinaryOperator, PostfixOperator, PrefixOperator};
use crate::parser::parse_result_ext::ParseResultExt;
use crate::parser::token_list::TokenList;
use crate::parser::token_list_ext::TokenListExt;
use crate::parser::{ParseErrorType, ParseResult};
use crate::token::TerminalToken;

pub fn prefix_operator(tokens: TokenList) -> ParseResult<PrefixOperator> {
    tokens
        .terminal(TerminalToken::Subtract)
        .map_val(PrefixOperator::Negate)
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Not)
                .map_val(PrefixOperator::LogicalNot)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::BitwiseNot)
                .map_val(PrefixOperator::BitwiseNot)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Typeof)
                .map_val(PrefixOperator::Typeof)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Clone)
                .map_val(PrefixOperator::Clone)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Delete)
                .map_val(PrefixOperator::Delete)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Increment)
                .map_val(PrefixOperator::Increment)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Decrement)
                .map_val(PrefixOperator::Decrement)
        })
        .or_error(|| tokens.error(ParseErrorType::ExpectedPrefixOperator))
}

pub fn postfix_operator(tokens: TokenList) -> ParseResult<PostfixOperator> {
    tokens
        .terminal(TerminalToken::Increment)
        .map_val(PostfixOperator::Increment)
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Decrement)
                .map_val(PostfixOperator::Decrement)
        })
        .or_error(|| tokens.error(ParseErrorType::ExpectedPostfixOperator))
}

pub fn binary_operator(tokens: TokenList) -> ParseResult<BinaryOperator> {
    tokens
        .terminal2(TerminalToken::Less, TerminalToken::Subtract)
        .map_val(|(a, b)| BinaryOperator::AssignNewSlot(a, b))
        .or_try(|| {
            tokens
                .terminal2(TerminalToken::Less, TerminalToken::Less)
                .map_val(|(a, b)| BinaryOperator::ShiftLeft(a, b))
        })
        .or_try(|| {
            tokens
                .terminal3(
                    TerminalToken::Greater,
                    TerminalToken::Greater,
                    TerminalToken::Greater,
                )
                .map_val(|(a, b, c)| BinaryOperator::UnsignedShiftRight(a, b, c))
        })
        .or_try(|| {
            tokens
                .terminal2(TerminalToken::Greater, TerminalToken::Greater)
                .map_val(|(a, b)| BinaryOperator::ShiftRight(a, b))
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Assign)
                .map_val(BinaryOperator::Assign)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::AddEqual)
                .map_val(BinaryOperator::AssignAdd)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::SubtractEqual)
                .map_val(BinaryOperator::AssignSubtract)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::MultiplyEqual)
                .map_val(BinaryOperator::AssignMultiply)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::DivideEqual)
                .map_val(BinaryOperator::AssignDivide)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::ModuloEqual)
                .map_val(BinaryOperator::AssignModulo)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Add)
                .map_val(BinaryOperator::Add)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Subtract)
                .map_val(BinaryOperator::Subtract)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Multiply)
                .map_val(BinaryOperator::Multiply)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Divide)
                .map_val(BinaryOperator::Divide)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Modulo)
                .map_val(BinaryOperator::Modulo)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Equal)
                .map_val(BinaryOperator::Equal)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::NotEqual)
                .map_val(BinaryOperator::NotEqual)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Less)
                .map_val(BinaryOperator::Less)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::LessEqual)
                .map_val(BinaryOperator::LessEqual)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Greater)
                .map_val(BinaryOperator::Greater)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::GreaterEqual)
                .map_val(BinaryOperator::GreaterEqual)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::ThreeWay)
                .map_val(BinaryOperator::ThreeWay)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::LogicalAnd)
                .map_val(BinaryOperator::LogicalAnd)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::LogicalOr)
                .map_val(BinaryOperator::LogicalOr)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::BitwiseAnd)
                .map_val(BinaryOperator::BitwiseAnd)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::BitwiseOr)
                .map_val(BinaryOperator::BitwiseOr)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::BitwiseXor)
                .map_val(BinaryOperator::BitwiseXor)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::In)
                .map_val(BinaryOperator::In)
        })
        .or_try(|| {
            tokens
                .terminal(TerminalToken::Instanceof)
                .map_val(BinaryOperator::Instanceof)
        })
        .or_error(|| tokens.error(ParseErrorType::ExpectedBinaryOperator))
}
