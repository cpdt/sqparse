// Based on http://www.squirrel-lang.org/doc/squirrel2.html#d0e1124
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Precedence {
    None = 0,

    // Comma operator, i.e `A, B, C`.
    Comma = 1,

    // Binary assignment operators like `A = B`, `A += B`, etc.
    Assignment = 2,

    // Expressions in a ternary operator, i.e `A ? B : C`.
    Ternary = 3,

    // `A || B`.
    LogicalOr = 4,

    // `A && B`, `A in B`, `A instanceof B`.
    TestOrLogicalAnd = 5,

    // `A | B`
    BitwiseOr = 6,

    // `A ^ B`
    BitwiseXor = 7,

    // `A & B`
    BitwiseAnd = 8,

    // Binary equality like `A == B` and `A != B`.
    Equality = 9,

    // Binary comparisons like `A < B`, `A <= B`, etc.
    Comparison = 10,

    // Bitshift operators like `<<`, `>>`, `>>>`.
    Bitshift = 11,

    // `A + B` and `A - B`
    AddSubtract = 12,

    // `A * B`, `A / B` and `A % B`
    MultiplyDivideModulo = 13,

    // Prefix operators like `-`, `~`, `!`, `typeof`, `++`, `--`, `delegate A : B`.
    Prefix = 14,

    // `A++`, `A--`, `A(...)`, `A[...]`.
    Postfix = 15,

    // `A.B`
    Property = 16,
}
