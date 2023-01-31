use crate::ast::Precedence;
use crate::token::Token;

/// A binary operator in a [`BinaryExpression`].
///
/// There are six categories of binary operator:
///  - Assignment - includes `=`, `<-`, `+=`, etc.
///  - Math - includes `+`, `-`, `*`, `/` and `%`.
///  - Comparison - includes `==`, `!=`, `<`, `<=`, etc.
///  - Logical - includes `&&` and `||`.
///  - Bitwise - includes `&`, `|`, `^`, `<<`, `>>` and `>>>`.
///  - Test - includes `in` and `instanceof`.
///
/// Each operator has a [precedence] which defines how it is parsed as part of a longer expression.
///
/// [`BinaryExpression`]: crate::ast::BinaryExpression
/// [precedence]: BinaryOperator::precedence
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOperator<'s> {
    /// Grammar: `=`
    Assign(&'s Token<'s>),
    /// Grammar: `<` `-`
    AssignNewSlot(&'s Token<'s>, &'s Token<'s>),
    /// Grammar: `+=`
    AssignAdd(&'s Token<'s>),
    /// Grammar: `-=`
    AssignSubtract(&'s Token<'s>),
    /// Grammar: `*=`
    AssignMultiply(&'s Token<'s>),
    /// Grammar: `/=`
    AssignDivide(&'s Token<'s>),
    /// Grammar: `%=`
    AssignModulo(&'s Token<'s>),

    /// Grammar: `+`
    Add(&'s Token<'s>),
    /// Grammar: `-`
    Subtract(&'s Token<'s>),
    /// Grammar: `*`
    Multiply(&'s Token<'s>),
    /// Grammar: `/`
    Divide(&'s Token<'s>),
    /// Grammar: `%`
    Modulo(&'s Token<'s>),

    /// Grammar: `==`
    Equal(&'s Token<'s>),
    /// Grammar: `!=`
    NotEqual(&'s Token<'s>),
    /// Grammar: `<`
    Less(&'s Token<'s>),
    /// Grammar: `<=`
    LessEqual(&'s Token<'s>),
    /// Grammar: `>`
    Greater(&'s Token<'s>),
    /// Grammar: `>=`
    GreaterEqual(&'s Token<'s>),
    /// Grammar: `<=>`
    ThreeWay(&'s Token<'s>),

    /// Grammar: `&&`
    LogicalAnd(&'s Token<'s>),
    /// Grammar: `||`
    LogicalOr(&'s Token<'s>),

    /// Grammar: `&`
    BitwiseAnd(&'s Token<'s>),
    /// Grammar: `|`
    BitwiseOr(&'s Token<'s>),
    /// Grammar: `^`
    BitwiseXor(&'s Token<'s>),
    /// Grammar: `<` `<`
    ShiftLeft(&'s Token<'s>, &'s Token<'s>),
    /// Grammar: `>` `>`
    ShiftRight(&'s Token<'s>, &'s Token<'s>),
    /// Grammar: `>` `>` `>`
    UnsignedShiftRight(&'s Token<'s>, &'s Token<'s>, &'s Token<'s>),

    /// Grammar: `in`
    In(&'s Token<'s>),
    /// Grammar: `instanceof`
    Instanceof(&'s Token<'s>),
}

/// A prefix operator in a [`PrefixExpression`].
///
/// All prefix operators share the [`Prefix`] precedence.
///
/// [`PrefixExpression`]: crate::ast::PrefixExpression
/// [`Prefix`]: Precedence::Prefix
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrefixOperator<'s> {
    /// Grammar: `-`
    Negate(&'s Token<'s>),
    /// Grammar: `!`
    LogicalNot(&'s Token<'s>),
    /// Grammar: `~`
    BitwiseNot(&'s Token<'s>),
    /// Grammar: `typeof`
    Typeof(&'s Token<'s>),
    /// Grammar: `clone`
    Clone(&'s Token<'s>),
    /// Grammar: `delete`
    Delete(&'s Token<'s>),
    /// Grammar: `++`
    Increment(&'s Token<'s>),
    /// Grammar: `--`
    Decrement(&'s Token<'s>),
}

/// A postfix operator in a [`PostfixExpression`].
///
/// All postfix operators share the [`Postfix`] precedence.
///
/// [`PostfixExpression`]: crate::ast::PostfixExpression
/// [`Postfix`]: Precedence::Postfix
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PostfixOperator<'s> {
    /// Grammar: `++`
    Increment(&'s Token<'s>),
    /// Grammar: `--`
    Decrement(&'s Token<'s>),
}

impl<'s> BinaryOperator<'s> {
    /// Returns the [`Precedence`] of the operator.
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
