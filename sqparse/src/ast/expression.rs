use crate::ast::{
    ClassDeclaration, FunctionDeclaration, Identifier, SeparatedList0, SeparatedList1, Type,
};
use crate::token::{LiteralToken, Token};

#[derive(Debug, Clone)]
pub enum Expression<'s> {
    Parens(ParensExpression<'s>),
    Literal(LiteralExpression<'s>),
    Var(Identifier<'s>),
    RootVar(RootVarExpression<'s>),

    Table(TableExpression<'s>),
    Class(ClassExpression<'s>),
    Array(ArrayExpression<'s>),

    // These all have sub-expressions and follow precedence rules
    // See http://www.squirrel-lang.org/doc/squirrel2.html#d0e1124
    Index(IndexExpression<'s>),
    Property(PropertyExpression<'s>),
    TernaryOperator(TernaryOperatorExpression<'s>),
    BinaryOperator(BinaryOperatorExpression<'s>),
    PostfixOperator(PostfixOperatorExpression<'s>),
    PrefixOperator(PrefixOperatorExpression<'s>),
    Comma(CommaExpression<'s>),
    Call(CallExpression<'s>),
    Function(FunctionExpression<'s>),
    Delegate(DelegateExpression<'s>),

    Vector(VectorExpression<'s>),
    Expect(ExpectExpression<'s>),
}

impl<'s> Expression<'s> {
    pub const NOT_COMMA_PRECEDENCE: u32 = 16;
    pub const NO_PRECEDENCE: u32 = u32::MAX;

    pub fn precedence(&self) -> u32 {
        match self {
            Expression::Function(_)
            | Expression::Parens(_)
            | Expression::Literal(_)
            | Expression::Vector(_)
            | Expression::Var(_)
            | Expression::RootVar(_)
            | Expression::Table(_)
            | Expression::Class(_)
            | Expression::Array(_)
            | Expression::Expect(_) => 0,

            Expression::Call(_)
            | Expression::Index(_)
            | Expression::Property(_) => 1,

            Expression::Delegate(_) => 2,
            Expression::PostfixOperator(_) => 3,
            Expression::PrefixOperator(_) => 4,

            Expression::BinaryOperator(BinaryOperatorExpression { ty: BinaryOperatorType::Divide(_) | BinaryOperatorType::Multiply(_) | BinaryOperatorType::Modulo(_), .. }) => 5,
            Expression::BinaryOperator(BinaryOperatorExpression { ty: BinaryOperatorType::Add(_) | BinaryOperatorType::Subtract(_), .. }) => 6,
            Expression::BinaryOperator(BinaryOperatorExpression { ty: BinaryOperatorType::ShiftLeft(_, _) | BinaryOperatorType::UnsignedShiftRight(_, _, _) | BinaryOperatorType::ShiftRight(_, _), .. }) => 7,
            Expression::BinaryOperator(BinaryOperatorExpression { ty: BinaryOperatorType::Less(_) | BinaryOperatorType::LessEqual(_) | BinaryOperatorType::Greater(_) | BinaryOperatorType::GreaterEqual(_) | BinaryOperatorType::ThreeWay(_), .. }) => 8,
            Expression::BinaryOperator(BinaryOperatorExpression { ty: BinaryOperatorType::Equal(_) | BinaryOperatorType::NotEqual(_), .. }) => 9,
            Expression::BinaryOperator(BinaryOperatorExpression { ty: BinaryOperatorType::BitwiseAnd(_), .. }) => 10,
            Expression::BinaryOperator(BinaryOperatorExpression { ty: BinaryOperatorType::BitwiseXor(_), .. }) => 11,
            Expression::BinaryOperator(BinaryOperatorExpression { ty: BinaryOperatorType::BitwiseOr(_), .. }) => 12,
            Expression::BinaryOperator(BinaryOperatorExpression { ty: BinaryOperatorType::LogicalAnd(_) | BinaryOperatorType::In(_) | BinaryOperatorType::Instanceof(_), .. }) => 13,
            Expression::BinaryOperator(BinaryOperatorExpression { ty: BinaryOperatorType::LogicalOr(_), .. }) => 14,

            Expression::TernaryOperator(_) => 15,

            Expression::BinaryOperator(BinaryOperatorExpression {
                                           ty: BinaryOperatorType::Assign(_)
                                           | BinaryOperatorType::AssignNewSlot(_, _)
                                           | BinaryOperatorType::AssignAdd(_)
                                           | BinaryOperatorType::AssignSubtract(_)
                                           | BinaryOperatorType::AssignMultiply(_)
                                           | BinaryOperatorType::AssignDivide(_)
                                           | BinaryOperatorType::AssignModulo(_), ..
                                       }) => 16,

            Expression::Comma(_) => 17,
        }
    }
}

//

#[derive(Debug, Clone)]
pub struct ParensExpression<'s> {
    pub value: Box<Expression<'s>>,

    pub open_token: &'s Token<'s>,
    pub close_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct LiteralExpression<'s> {
    pub literal: LiteralToken<'s>,
    pub token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct RootVarExpression<'s> {
    pub name: Identifier<'s>,

    pub root_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct IndexExpression<'s> {
    pub base: Box<Expression<'s>>,
    pub index: Box<Expression<'s>>,

    pub open_index_token: &'s Token<'s>,
    pub close_index_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct PropertyExpression<'s> {
    pub base: Box<Expression<'s>>,
    pub property: Identifier<'s>,

    pub separator_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct TernaryOperatorExpression<'s> {
    pub condition: Box<Expression<'s>>,
    pub true_value: Box<Expression<'s>>,
    pub false_value: Box<Expression<'s>>,

    pub question_token: &'s Token<'s>,
    pub colon_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct BinaryOperatorExpression<'s> {
    pub ty: BinaryOperatorType<'s>,
    pub left_value: Box<Expression<'s>>,
    pub right_value: Box<Expression<'s>>,
}

#[derive(Debug, Clone)]
pub struct PrefixOperatorExpression<'s> {
    pub ty: PrefixOperatorType,
    pub value: Box<Expression<'s>>,

    pub operator_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct PostfixOperatorExpression<'s> {
    pub ty: PostfixOperatorType,
    pub value: Box<Expression<'s>>,

    pub operator_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct CommaExpression<'s> {
    pub values: SeparatedList1<'s, Expression<'s>>,
}

#[derive(Debug, Clone)]
pub struct TableExpression<'s> {
    pub slots: Vec<TableSlot<'s>>,

    pub open_token: &'s Token<'s>,
    pub spread_token: Option<&'s Token<'s>>,
    pub close_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct ClassExpression<'s> {
    pub class: ClassDeclaration<'s>,

    pub class_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct ArrayExpression<'s> {
    pub values: Vec<ArrayValue<'s>>,

    pub open_token: &'s Token<'s>,
    pub spread_token: Option<&'s Token<'s>>,
    pub close_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct FunctionExpression<'s> {
    pub return_type: Option<Type<'s>>,
    pub declaration: FunctionDeclaration<'s>,

    pub function_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct CallExpression<'s> {
    pub function: Box<Expression<'s>>,
    pub arguments: SeparatedList0<'s, Expression<'s>>,
    pub post_initializer: Option<TableExpression<'s>>,

    pub open_arguments_token: &'s Token<'s>,
    pub trailing_separator_token: Option<&'s Token<'s>>,
    pub close_arguments_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct DelegateExpression<'s> {
    pub base: Box<Expression<'s>>,
    pub delegate: Box<Expression<'s>>,

    pub delegate_token: &'s Token<'s>,
    pub separator_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct VectorExpression<'s> {
    pub x_val: Box<Expression<'s>>,
    pub y_val: Box<Expression<'s>>,
    pub z_val: Box<Expression<'s>>,

    pub open_token: &'s Token<'s>,
    pub first_separator_token: &'s Token<'s>,
    pub second_separator_token: &'s Token<'s>,
    pub close_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct ExpectExpression<'s> {
    pub expected_type: Type<'s>,
    pub value: Box<Expression<'s>>,

    pub expect_token: &'s Token<'s>,
    pub open_value_token: &'s Token<'s>,
    pub close_value_token: &'s Token<'s>,
}

//

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperatorType<'s> {
    Assign(&'s Token<'s>),
    AssignNewSlot(&'s Token<'s>, &'s Token<'s>),
    AssignAdd(&'s Token<'s>),
    AssignSubtract(&'s Token<'s>),
    AssignMultiply(&'s Token<'s>),
    AssignDivide(&'s Token<'s>),
    AssignModulo(&'s Token<'s>),

    Add(&'s Token<'s>),
    Subtract(&'s Token<'s>),
    Multiply(&'s Token<'s>),
    Divide(&'s Token<'s>),
    Modulo(&'s Token<'s>),

    Equal(&'s Token<'s>),
    NotEqual(&'s Token<'s>),
    Less(&'s Token<'s>),
    LessEqual(&'s Token<'s>),
    Greater(&'s Token<'s>),
    GreaterEqual(&'s Token<'s>),
    ThreeWay(&'s Token<'s>),

    LogicalAnd(&'s Token<'s>),
    LogicalOr(&'s Token<'s>),

    BitwiseAnd(&'s Token<'s>),
    BitwiseOr(&'s Token<'s>),
    BitwiseXor(&'s Token<'s>),
    ShiftLeft(&'s Token<'s>, &'s Token<'s>),
    ShiftRight(&'s Token<'s>, &'s Token<'s>),
    UnsignedShiftRight(&'s Token<'s>, &'s Token<'s>, &'s Token<'s>),

    In(&'s Token<'s>),
    Instanceof(&'s Token<'s>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefixOperatorType {
    Negate,
    LogicalNot,
    BitwiseNot,
    Typeof,
    Clone,
    Delete,
    Increment,
    Decrement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostfixOperatorType {
    Increment,
    Decrement,
}

#[derive(Debug, Clone)]
pub struct TableSlot<'s> {
    pub ty: TableSlotType<'s>,

    pub separator_token: Option<&'s Token<'s>>,
}

#[derive(Debug, Clone)]
pub struct ArrayValue<'s> {
    pub value: Box<Expression<'s>>,

    pub separator_token: Option<&'s Token<'s>>,
}

//

#[derive(Debug, Clone)]
pub enum TableSlotType<'s> {
    Property {
        name: Identifier<'s>,
        value: Box<Expression<'s>>,

        separator_token: &'s Token<'s>,
    },
    ComputedProperty {
        name: Box<Expression<'s>>,
        value: Box<Expression<'s>>,

        open_name_token: &'s Token<'s>,
        close_name_token: &'s Token<'s>,
        separator_token: &'s Token<'s>,
    },
    JsonProperty {
        name: LiteralExpression<'s>,
        value: Box<Expression<'s>>,

        separator_token: &'s Token<'s>,
    },
    Function {
        return_type: Option<Type<'s>>,
        name: Identifier<'s>,
        function: Box<FunctionDeclaration<'s>>,

        function_token: &'s Token<'s>,
    },
}
