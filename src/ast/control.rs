use crate::ast::{Expression, Identifier, Statement, Type, VarDeclarationStatement};
use crate::token::Token;

/// Optional `else` part of an [`IfStatement`].
///
/// Grammar: `else` [Statement]
///
/// [`IfStatement`]: crate::ast::IfStatement
#[derive(Debug, Clone)]
pub struct IfElse<'s> {
    pub else_: &'s Token<'s>,
    pub body: Box<Statement<'s>>,
}

/// Case block in a [`SwitchStatement`].
///
/// Grammar: [SwitchCaseCondition] `:` [Statement]*
///
/// [`SwitchStatement`]: crate::ast::SwitchStatement
#[derive(Debug, Clone)]
pub struct SwitchCase<'s> {
    pub condition: SwitchCaseCondition<'s>,
    pub colon: &'s Token<'s>,
    pub body: Vec<Statement<'s>>,
}

/// Condition in a [`SwitchCase`].
#[derive(Debug, Clone)]
pub enum SwitchCaseCondition<'s> {
    /// Default case.
    ///
    /// Grammar: `default`
    Default { default: &'s Token<'s> },

    /// Specific case.
    ///
    /// Grammar: `case` [Expression]
    Case {
        case: &'s Token<'s>,
        value: Box<Expression<'s>>,
    },
}

/// Declaration part of a [`ForStatement`].
///
/// [`ForStatement`]: crate::ast::ForStatement
#[derive(Debug, Clone)]
pub enum ForDeclaration<'s> {
    /// Expression declaration.
    ///
    /// Grammar: [Expression]
    Expression(Box<Expression<'s>>),

    /// Variable declaration.
    ///
    /// Grammar: [VarDeclarationStatement]
    Declaration(VarDeclarationStatement<'s>),
}

/// Optional index declaration in a [`ForeachStatement`].
///
/// Grammar: [Type]? [Identifier] `,`
///
/// [`ForeachStatement`]: crate::ast::ForeachStatement
#[derive(Debug, Clone)]
pub struct ForeachIndex<'s> {
    pub ty: Option<Type<'s>>,
    pub name: Identifier<'s>,
    pub comma: &'s Token<'s>,
}
