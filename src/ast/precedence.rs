// Based on http://www.squirrel-lang.org/doc/squirrel2.html#d0e1124

/// Table of precedence values for different kinds of expression operators.
///
/// The higher the precedence, the closer that operator will "bind" its directly adjacent
/// expressions.
///
/// For example, the expression `A + B . C` is parsed as `A + (B . C)` because the [Property]
/// precedence is higher than [AddSubtract]. If it was not, the expression could parse as
/// `(A + B) . C`.
///
/// [Property]: Precedence::Property
/// [AddSubtract]: Precedence::AddSubtract
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Precedence {
    /// Reserved value for the precedence of the end of the input.
    None = 0,

    /// Comma operator, i.e `A, B, C`.
    Comma = 1,

    /// Binary assignment operators like `A = B`, `A += B`, etc.
    Assignment = 2,

    /// Expressions in a ternary operator, i.e `A ? B : C`.
    Ternary = 3,

    /// `A || B`.
    LogicalOr = 4,

    /// `A && B`, `A in B`, `A instanceof B`.
    TestOrLogicalAnd = 5,

    /// `A | B`
    BitwiseOr = 6,

    /// `A ^ B`
    BitwiseXor = 7,

    /// `A & B`
    BitwiseAnd = 8,

    /// Binary equality like `A == B` and `A != B`.
    Equality = 9,

    /// Binary comparisons like `A < B`, `A <= B`, etc.
    Comparison = 10,

    /// Bitshift operators like `<<`, `>>`, `>>>`.
    Bitshift = 11,

    /// `A + B` and `A - B`
    AddSubtract = 12,

    /// `A * B`, `A / B` and `A % B`
    MultiplyDivideModulo = 13,

    /// Prefix operators like `-`, `~`, `!`, `typeof`, `++`, `--`, `delegate A : B`.
    Prefix = 14,

    /// `A++`, `A--`, `A(...)`, `A[...]`.
    Postfix = 15,

    /// `A.B`
    Property = 16,
}
