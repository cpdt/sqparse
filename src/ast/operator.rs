use crate::ast::Precedence;
use crate::token::Token;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOperator<'s> {
    // `=`
    Assign(&'s Token<'s>),
    // `<` `-`
    AssignNewSlot(&'s Token<'s>, &'s Token<'s>),
    // `+=`
    AssignAdd(&'s Token<'s>),
    // `-=`
    AssignSubtract(&'s Token<'s>),
    // `*=`
    AssignMultiply(&'s Token<'s>),
    // `/=`
    AssignDivide(&'s Token<'s>),
    // `%=`
    AssignModulo(&'s Token<'s>),

    // `+`
    Add(&'s Token<'s>),
    // `-`
    Subtract(&'s Token<'s>),
    // `*`
    Multiply(&'s Token<'s>),
    // `/`
    Divide(&'s Token<'s>),
    // `%`
    Modulo(&'s Token<'s>),

    // `==`
    Equal(&'s Token<'s>),
    // `!=`
    NotEqual(&'s Token<'s>),
    // `<`
    Less(&'s Token<'s>),
    // `<=`
    LessEqual(&'s Token<'s>),
    // `>`
    Greater(&'s Token<'s>),
    // `>=`
    GreaterEqual(&'s Token<'s>),
    // `<=>`
    ThreeWay(&'s Token<'s>),

    // `&&`
    LogicalAnd(&'s Token<'s>),
    // `||`
    LogicalOr(&'s Token<'s>),

    // `&`
    BitwiseAnd(&'s Token<'s>),
    // `|`
    BitwiseOr(&'s Token<'s>),
    // `^`
    BitwiseXor(&'s Token<'s>),
    // `<<`
    ShiftLeft(&'s Token<'s>, &'s Token<'s>),
    // `>>`
    ShiftRight(&'s Token<'s>, &'s Token<'s>),
    // `>>>`
    UnsignedShiftRight(&'s Token<'s>, &'s Token<'s>, &'s Token<'s>),

    // `in`
    In(&'s Token<'s>),
    // `instanceof`
    Instanceof(&'s Token<'s>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrefixOperator<'s> {
    // `-`
    Negate(&'s Token<'s>),
    // `!`
    LogicalNot(&'s Token<'s>),
    // `~`
    BitwiseNot(&'s Token<'s>),
    // `typeof`
    Typeof(&'s Token<'s>),
    // `clone`
    Clone(&'s Token<'s>),
    // `delete`
    Delete(&'s Token<'s>),
    // `++`
    Increment(&'s Token<'s>),
    // `--`
    Decrement(&'s Token<'s>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PostfixOperator<'s> {
    // `++`
    Increment(&'s Token<'s>),
    // `--`
    Decrement(&'s Token<'s>),
}

impl<'s> BinaryOperator<'s> {
    pub fn precedence(self) -> Precedence {
        match self {
            BinaryOperator::Assign(_)
            | BinaryOperator::AssignNewSlot(_, _)
            | BinaryOperator::AssignAdd(_)
            | BinaryOperator::AssignSubtract(_)
            | BinaryOperator::AssignMultiply(_)
            | BinaryOperator::AssignDivide(_)
            | BinaryOperator::AssignModulo(_) => Precedence::Assignment,

            BinaryOperator::LogicalOr(_) => Precedence::LogicalOr,

            BinaryOperator::LogicalAnd(_)
            | BinaryOperator::In(_)
            | BinaryOperator::Instanceof(_) => Precedence::TestOrLogicalAnd,

            BinaryOperator::BitwiseOr(_) => Precedence::BitwiseOr,
            BinaryOperator::BitwiseXor(_) => Precedence::BitwiseXor,
            BinaryOperator::BitwiseAnd(_) => Precedence::BitwiseAnd,

            BinaryOperator::Equal(_) | BinaryOperator::NotEqual(_) => Precedence::Equality,

            BinaryOperator::Less(_)
            | BinaryOperator::LessEqual(_)
            | BinaryOperator::Greater(_)
            | BinaryOperator::GreaterEqual(_)
            | BinaryOperator::ThreeWay(_) => Precedence::Comparison,

            BinaryOperator::ShiftLeft(_, _)
            | BinaryOperator::ShiftRight(_, _)
            | BinaryOperator::UnsignedShiftRight(_, _, _) => Precedence::Bitshift,

            BinaryOperator::Add(_) | BinaryOperator::Subtract(_) => Precedence::AddSubtract,

            BinaryOperator::Multiply(_) | BinaryOperator::Divide(_) | BinaryOperator::Modulo(_) => {
                Precedence::MultiplyDivideModulo
            }
        }
    }
}
