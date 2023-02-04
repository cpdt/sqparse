use crate::ast::{
    ArrayValue, BinaryOperator, ClassDefinition, FunctionDefinition, Identifier, MethodIdentifier,
    PostfixOperator, PrefixOperator, SeparatedList1, SeparatedListTrailing0, TableSlot, Type,
};
use crate::token::{LiteralToken, Token};

/// An expression.
///
/// Many types of expressions are recursive, containing other expressions. [Precedence] is used to
/// determine how to parse these expressions.
///
/// [Precedence]: crate::ast::Precedence
#[derive(Debug, Clone)]
pub enum Expression<'s> {
    Parens(ParensExpression<'s>),
    Literal(LiteralExpression<'s>),
    Var(VarExpression<'s>),
    RootVar(RootVarExpression<'s>),
    Index(IndexExpression<'s>),
    Property(PropertyExpression<'s>),
    Ternary(TernaryExpression<'s>),
    Binary(BinaryExpression<'s>),
    Prefix(PrefixExpression<'s>),
    Postfix(PostfixExpression<'s>),
    Comma(CommaExpression<'s>),
    Table(TableExpression<'s>),
    Class(ClassExpression<'s>),
    Array(ArrayExpression<'s>),
    Function(FunctionExpression<'s>),
    Call(CallExpression<'s>),
    Delegate(DelegateExpression<'s>),
    Vector(VectorExpression<'s>),
    Expect(ExpectExpression<'s>),
}

/// An expression enclosed in parentheses.
///
/// Grammar: `(` [Expression] `)`
#[derive(Debug, Clone)]
pub struct ParensExpression<'s> {
    pub open: &'s Token<'s>,
    pub value: Box<Expression<'s>>,
    pub close: &'s Token<'s>,
}

/// A single literal value.
///
/// Grammar: [LiteralToken]
#[derive(Debug, Clone)]
pub struct LiteralExpression<'s> {
    pub literal: LiteralToken<'s>,
    pub token: &'s Token<'s>,
}

/// A variable reference.
///
/// Grammar: [Identifier]
#[derive(Debug, Clone)]
pub struct VarExpression<'s> {
    pub name: Identifier<'s>,
}

/// A reference to a variable in the root table.
///
/// Grammar: `::` [Identifier]
#[derive(Debug, Clone)]
pub struct RootVarExpression<'s> {
    pub root: &'s Token<'s>,
    pub name: Identifier<'s>,
}

/// An expression with a prefixed operator.
///
/// Grammar: [PrefixOperator] [Expression]
#[derive(Debug, Clone)]
pub struct PrefixExpression<'s> {
    pub operator: PrefixOperator<'s>,
    pub value: Box<Expression<'s>>,
}

/// A table literal.
///
/// Grammar: `{` [TableSlot]* `...`? `}`
#[derive(Debug, Clone)]
pub struct TableExpression<'s> {
    pub open: &'s Token<'s>,
    pub slots: Vec<TableSlot<'s>>,
    pub spread: Option<&'s Token<'s>>,
    pub close: &'s Token<'s>,
}

/// A class literal.
///
/// Grammar: `class` [ClassDefinition]
#[derive(Debug, Clone)]
pub struct ClassExpression<'s> {
    pub class: &'s Token<'s>,
    pub definition: ClassDefinition<'s>,
}

/// An array literal.
///
/// Grammar: `[` [ArrayValue]* `...`? `]`
#[derive(Debug, Clone)]
pub struct ArrayExpression<'s> {
    pub open: &'s Token<'s>,
    pub values: Vec<ArrayValue<'s>>,
    pub spread: Option<&'s Token<'s>>,
    pub close: &'s Token<'s>,
}

/// A function literal.
///
/// Grammar: [Type]? `function` [FunctionDefinition]
#[derive(Debug, Clone)]
pub struct FunctionExpression<'s> {
    pub return_type: Option<Type<'s>>,
    pub function: &'s Token<'s>,
    pub definition: FunctionDefinition<'s>,
}

/// A delegate expression.
///
/// Grammar: `delegate` [Expression] `:` [Expression]
#[derive(Debug, Clone)]
pub struct DelegateExpression<'s> {
    pub delegate: &'s Token<'s>,
    pub parent: Box<Expression<'s>>,
    pub colon: &'s Token<'s>,
    pub value: Box<Expression<'s>>,
}

/// A vector literal.
///
/// Grammar: `<` [Expression] `,` [Expression] `,` [Expression] `>`
#[derive(Debug, Clone)]
pub struct VectorExpression<'s> {
    pub open: &'s Token<'s>,
    pub x: Box<Expression<'s>>,
    pub comma_1: &'s Token<'s>,
    pub y: Box<Expression<'s>>,
    pub comma_2: &'s Token<'s>,
    pub z: Box<Expression<'s>>,
    pub close: &'s Token<'s>,
}

/// An expect expression.
///
/// Grammar: `expect` [Type] `(` [Expression] `)`
#[derive(Debug, Clone)]
pub struct ExpectExpression<'s> {
    pub expect: &'s Token<'s>,
    pub ty: Type<'s>,
    pub open: &'s Token<'s>,
    pub value: Box<Expression<'s>>,
    pub close: &'s Token<'s>,
}

/// An index expression.
///
/// Grammar: [Expression] `[` [Expression] `]`
#[derive(Debug, Clone)]
pub struct IndexExpression<'s> {
    pub base: Box<Expression<'s>>,
    pub open: &'s Token<'s>,
    pub index: Box<Expression<'s>>,
    pub close: &'s Token<'s>,
}

/// A property access expression.
///
/// Grammar: [Expression] `.` [MethodIdentifier]
#[derive(Debug, Clone)]
pub struct PropertyExpression<'s> {
    pub base: Box<Expression<'s>>,
    pub dot: &'s Token<'s>,
    pub property: MethodIdentifier<'s>,
}

/// A ternary expression.
///
/// Grammar: [Expression] `?` [Expression] `:` [Expression]
#[derive(Debug, Clone)]
pub struct TernaryExpression<'s> {
    pub condition: Box<Expression<'s>>,
    pub question: &'s Token<'s>,
    pub true_value: Box<Expression<'s>>,
    pub separator: &'s Token<'s>,
    pub false_value: Box<Expression<'s>>,
}

/// A binary expression - assignment, comparison, test, or math.
///
/// Grammar: [Expression] [BinaryOperator] [Expression]
#[derive(Debug, Clone)]
pub struct BinaryExpression<'s> {
    pub left: Box<Expression<'s>>,
    pub operator: BinaryOperator<'s>,
    pub right: Box<Expression<'s>>,
}

/// An expression with a postfix operator.
///
/// Grammar: [Expression] [PostfixOperator]
#[derive(Debug, Clone)]
pub struct PostfixExpression<'s> {
    pub value: Box<Expression<'s>>,
    pub operator: PostfixOperator<'s>,
}

/// A function call expression.
///
/// Grammar: [Expression] `(` [SeparatedListTrailing0]<[Expression] `,`> `)` [TableExpression]?
#[derive(Debug, Clone)]
pub struct CallExpression<'s> {
    pub function: Box<Expression<'s>>,
    pub open: &'s Token<'s>,
    pub arguments: SeparatedListTrailing0<'s, Expression<'s>>,
    pub close: &'s Token<'s>,
    pub post_initializer: Option<TableExpression<'s>>,
}

/// A comma expression, with two or more sub-expressions.
///
/// Grammar: [SeparatedList1]<[Expression] `,`>
#[derive(Debug, Clone)]
pub struct CommaExpression<'s> {
    pub values: SeparatedList1<'s, Expression<'s>>,
}
