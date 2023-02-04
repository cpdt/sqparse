use crate::ast::{Expression, Identifier, Statement, StatementType, Type, VarDefinitionStatement};
use crate::token::Token;

/// Trailing part of an [`IfStatement`].
///
/// Grammar: [StatementType] | ([Statement] `else` [StatementType])
///
/// [`IfStatement`]: crate::ast::IfStatement
#[derive(Debug, Clone)]
pub enum IfStatementType<'s> {
    NoElse {
        body: Box<StatementType<'s>>,
    },
    Else {
        body: Box<Statement<'s>>,
        else_: &'s Token<'s>,
        else_body: Box<StatementType<'s>>,
    },
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

/// Definition part of a [`ForStatement`].
///
/// [`ForStatement`]: crate::ast::ForStatement
#[derive(Debug, Clone)]
pub enum ForDefinition<'s> {
    /// Expression definition.
    ///
    /// Grammar: [Expression]
    Expression(Box<Expression<'s>>),

    /// Variable definition.
    ///
    /// Grammar: [VarDefinitionStatement]
    Definition(VarDefinitionStatement<'s>),
}

/// Optional index definition in a [`ForeachStatement`].
///
/// Grammar: [Type]? [Identifier] `,`
///
/// [`ForeachStatement`]: crate::ast::ForeachStatement
#[derive(Debug, Clone)]
pub struct ForeachIndex<'s> {
    pub type_: Option<Type<'s>>,
    pub name: Identifier<'s>,
    pub comma: &'s Token<'s>,
}
