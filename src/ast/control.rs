use crate::ast::{Expression, Identifier, Statement, Type, VarDeclarationStatement};
use crate::token::Token;

// `else` Statement
#[derive(Debug, Clone)]
pub struct IfElse<'s> {
    pub else_: &'s Token<'s>,
    pub body: Box<Statement<'s>>,
}

// SwitchCaseCondition `:` Statement+
#[derive(Debug, Clone)]
pub struct SwitchCase<'s> {
    pub condition: SwitchCaseCondition<'s>,
    pub colon: &'s Token<'s>,
    pub body: Vec<Statement<'s>>,
}

#[derive(Debug, Clone)]
pub enum SwitchCaseCondition<'s> {
    // `default`
    Default {
        default: &'s Token<'s>,
    },

    // `case` Expression
    Case {
        case: &'s Token<'s>,
        value: Box<Expression<'s>>,
    },
}

#[derive(Debug, Clone)]
pub enum ForDeclaration<'s> {
    // Expression
    Expression(Box<Expression<'s>>),

    // VarDeclarationStatement
    Declaration(VarDeclarationStatement<'s>),
}

// Type? Identifier `,`
#[derive(Debug, Clone)]
pub struct ForeachIndex<'s> {
    pub ty: Option<Type<'s>>,
    pub name: Identifier<'s>,
    pub comma: &'s Token<'s>,
}
