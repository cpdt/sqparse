use crate::ast::{
    ClassDeclaration, EnumEntry, Expression, ForDeclaration, ForeachIndex, FunctionDeclaration,
    GlobalDeclaration, Identifier, IfElse, SeparatedList1, SeparatedListTrailing1,
    StructDeclaration, SwitchCase, Type, VarDeclaration, VarInitializer,
};
use crate::token::Token;

/// A statement.
///
/// A Squirrel program is made up of a list of statements. Statements are either separated by
/// semicolons (`;`) or newlines.
///
/// Grammar: [StatementType] `;`?
#[derive(Debug, Clone)]
pub struct Statement<'s> {
    pub ty: StatementType<'s>,
    pub end: Option<&'s Token<'s>>,
}

/// A statement, excluding a trailing semicolon.
#[derive(Debug, Clone)]
pub enum StatementType<'s> {
    Empty(EmptyStatement<'s>),
    Block(BlockStatement<'s>),
    If(IfStatement<'s>),
    While(WhileStatement<'s>),
    DoWhile(DoWhileStatement<'s>),
    Switch(SwitchStatement<'s>),
    For(ForStatement<'s>),
    ForeachStatement(ForeachStatement<'s>),
    Break(BreakStatement<'s>),
    Continue(ContinueStatement<'s>),
    Return(ReturnStatement<'s>),
    Yield(YieldStatement<'s>),
    VarDeclaration(VarDeclarationStatement<'s>),
    ConstructorDeclaration(ConstructorDeclarationStatement<'s>),
    FunctionDeclaration(FunctionDeclarationStatement<'s>),
    ClassDeclaration(ClassDeclarationStatement<'s>),
    TryCatch(TryCatchStatement<'s>),
    Throw(ThrowStatement<'s>),
    Const(ConstStatement<'s>),
    Enum(EnumStatement<'s>),
    Expression(ExpressionStatement<'s>),

    // _re additions
    Thread(ThreadStatement<'s>),
    DelayThread(DelayThreadStatement<'s>),
    WaitThread(WaitThreadStatement<'s>),
    Wait(WaitStatement<'s>),
    StructDeclaration(StructDeclarationStatement<'s>),
    Typedef(TypedefStatement<'s>),
    Global(GlobalStatement<'s>),
    GlobalizeAllFunctions(GlobalizeAllFunctionsStatement<'s>),
    Untyped(UntypedStatement<'s>),
}

/// An empty statement.
///
/// Grammar: &lt;empty>?
#[derive(Debug, Clone)]
pub struct EmptyStatement<'s> {
    pub empty: Option<&'s Token<'s>>,
}

/// A block statement.
///
/// Grammar: `{` [Statement]+ `}`
#[derive(Debug, Clone)]
pub struct BlockStatement<'s> {
    pub open: &'s Token<'s>,
    pub statements: Vec<Statement<'s>>,
    pub close: &'s Token<'s>,
}

/// An `if` statement, with an optional `else` block.
///
/// Grammar: `if` `(` [Expression] `)` [Statement] [IfElse]?
#[derive(Debug, Clone)]
pub struct IfStatement<'s> {
    pub if_: &'s Token<'s>,
    pub open: &'s Token<'s>,
    pub condition: Box<Expression<'s>>,
    pub close: &'s Token<'s>,
    pub body: Box<Statement<'s>>,
    pub else_: Option<IfElse<'s>>,
}

/// A `while` statement.
///
/// Grammar: `while` `(` [Expression] `)` [Statement]
#[derive(Debug, Clone)]
pub struct WhileStatement<'s> {
    pub while_: &'s Token<'s>,
    pub open: &'s Token<'s>,
    pub condition: Box<Expression<'s>>,
    pub close: &'s Token<'s>,
    pub body: Box<Statement<'s>>,
}

/// A `do-while` statement.
///
/// Grammar: `do` [Statement] `while` `(` [Expression] `)`
#[derive(Debug, Clone)]
pub struct DoWhileStatement<'s> {
    pub do_: &'s Token<'s>,
    pub body: Box<Statement<'s>>,
    pub while_: &'s Token<'s>,
    pub open: &'s Token<'s>,
    pub condition: Box<Expression<'s>>,
    pub close: &'s Token<'s>,
}

/// A `switch` statement.
///
/// Grammar: `switch` `(` [Expression] `)` `{` [SwitchCase]* `}`
#[derive(Debug, Clone)]
pub struct SwitchStatement<'s> {
    pub switch: &'s Token<'s>,
    pub open_condition: &'s Token<'s>,
    pub condition: Box<Expression<'s>>,
    pub close_condition: &'s Token<'s>,
    pub open_cases: &'s Token<'s>,
    pub cases: Vec<SwitchCase<'s>>,
    pub close_cases: &'s Token<'s>,
}

/// A `for` loop statement.
///
/// Grammar: `for` `(` [ForDeclaration]? `;` [Expression]? `;` [Expression]? `)` [Statement]
#[derive(Debug, Clone)]
pub struct ForStatement<'s> {
    pub for_: &'s Token<'s>,
    pub open: &'s Token<'s>,
    pub initializer: Option<ForDeclaration<'s>>,
    pub semicolon_1: &'s Token<'s>,
    pub condition: Option<Box<Expression<'s>>>,
    pub semicolon_2: &'s Token<'s>,
    pub increment: Option<Box<Expression<'s>>>,
    pub close: &'s Token<'s>,
    pub body: Box<Statement<'s>>,
}

/// A `foreach` loop statement.
///
/// Grammar: `foreach` `(` [ForeachIndex]? [Type]? [Identifier] `in` [Expression] `)` [Statement]
#[derive(Debug, Clone)]
pub struct ForeachStatement<'s> {
    pub foreach: &'s Token<'s>,
    pub open: &'s Token<'s>,
    pub index: Option<ForeachIndex<'s>>,
    pub value_type: Option<Type<'s>>,
    pub value_name: Identifier<'s>,
    pub in_: &'s Token<'s>,
    pub array: Box<Expression<'s>>,
    pub close: &'s Token<'s>,
    pub body: Box<Statement<'s>>,
}

/// A `break` statement.
///
/// Grammar: `break`
#[derive(Debug, Clone)]
pub struct BreakStatement<'s> {
    pub break_: &'s Token<'s>,
}

/// A `continue` statement.
///
/// Grammar: `continue`
#[derive(Debug, Clone)]
pub struct ContinueStatement<'s> {
    pub continue_: &'s Token<'s>,
}

/// A `return` statement.
///
/// Grammar: `return` [Expression]?
#[derive(Debug, Clone)]
pub struct ReturnStatement<'s> {
    pub return_: &'s Token<'s>,
    pub value: Option<Box<Expression<'s>>>,
}

/// A `yield` statement.
///
/// Grammar: `yield` [Expression]?
#[derive(Debug, Clone)]
pub struct YieldStatement<'s> {
    pub yield_: &'s Token<'s>,
    pub value: Option<Box<Expression<'s>>>,
}

/// A variable declaration statement.
///
/// Grammar: [Type] [SeparatedListTrailing1]<[VarDeclaration] `,`>
#[derive(Debug, Clone)]
pub struct VarDeclarationStatement<'s> {
    pub ty: Type<'s>,
    pub declarations: SeparatedListTrailing1<'s, VarDeclaration<'s>>,
}

/// An out-of-band constructor declaration statement.
///
/// Grammar: `function` ([Identifier] `::`)* `constructor` [FunctionDeclaration]
#[derive(Debug, Clone)]
pub struct ConstructorDeclarationStatement<'s> {
    pub function: &'s Token<'s>,
    pub namespaces: Vec<(Identifier<'s>, &'s Token<'s>)>,
    pub last_name: Identifier<'s>,
    pub last_namespace: &'s Token<'s>,
    pub constructor: &'s Token<'s>,
    pub declaration: FunctionDeclaration<'s>,
}

/// A function declaration statement.
///
/// Grammar: [Type]? `function` [SeparatedList1]<[Identifier] `::`> [FunctionDeclaration]
#[derive(Debug, Clone)]
pub struct FunctionDeclarationStatement<'s> {
    pub return_type: Option<Type<'s>>,
    pub function: &'s Token<'s>,
    pub name: SeparatedList1<'s, Identifier<'s>>,
    pub declaration: FunctionDeclaration<'s>,
}

/// A class declaration statement.
///
/// Grammar: `class` [Expression] [ClassDeclaration]
#[derive(Debug, Clone)]
pub struct ClassDeclarationStatement<'s> {
    pub class: &'s Token<'s>,
    pub name: Box<Expression<'s>>,
    pub declaration: ClassDeclaration<'s>,
}

/// A `try-catch` statement.
///
/// Grammar: `try` [Statement] `catch` `(` [Identifier] `)` [Statement]
#[derive(Debug, Clone)]
pub struct TryCatchStatement<'s> {
    pub try_: &'s Token<'s>,
    pub body: Box<Statement<'s>>,
    pub catch: &'s Token<'s>,
    pub open: &'s Token<'s>,
    pub catch_name: Identifier<'s>,
    pub close: &'s Token<'s>,
    pub catch_body: Box<Statement<'s>>,
}

/// A `throw` statement.
///
/// Grammar: `throw` [Expression]
#[derive(Debug, Clone)]
pub struct ThrowStatement<'s> {
    pub throw: &'s Token<'s>,
    pub value: Box<Expression<'s>>,
}

/// A `const` declaration statement.
///
/// Grammar: `const` [Type]? [Identifier] [VarInitializer]
#[derive(Debug, Clone)]
pub struct ConstStatement<'s> {
    pub const_: &'s Token<'s>,
    pub const_type: Option<Type<'s>>,
    pub name: Identifier<'s>,
    pub initializer: VarInitializer<'s>,
}

/// An `enum` declaration statement.
///
/// Grammar: `enum` [Identifier] `{` [EnumEntry]* `}`
#[derive(Debug, Clone)]
pub struct EnumStatement<'s> {
    pub enum_: &'s Token<'s>,
    pub name: Identifier<'s>,
    pub open: &'s Token<'s>,
    pub entries: Vec<EnumEntry<'s>>,
    pub close: &'s Token<'s>,
}

/// An expression statement.
///
/// Grammar: [Expression]
#[derive(Debug, Clone)]
pub struct ExpressionStatement<'s> {
    pub value: Box<Expression<'s>>,
}

/// A `thread` statement.
///
/// Grammar: `thread` [Expression]
#[derive(Debug, Clone)]
pub struct ThreadStatement<'s> {
    pub thread: &'s Token<'s>,
    pub value: Box<Expression<'s>>,
}

/// A `delaythread` statement.
///
/// Grammar: `delaythread` `(` [Expression] `)` [Expression]
#[derive(Debug, Clone)]
pub struct DelayThreadStatement<'s> {
    pub delay_thread: &'s Token<'s>,
    pub open: &'s Token<'s>,
    pub duration: Box<Expression<'s>>,
    pub close: &'s Token<'s>,
    pub value: Box<Expression<'s>>,
}

/// A `waitthread` statement.
///
/// Grammar: `waitthread` [Expression]
#[derive(Debug, Clone)]
pub struct WaitThreadStatement<'s> {
    pub wait_thread: &'s Token<'s>,
    pub value: Box<Expression<'s>>,
}

/// A `wait` statement.
///
/// Grammar: `wait` [Expression]
#[derive(Debug, Clone)]
pub struct WaitStatement<'s> {
    pub wait: &'s Token<'s>,
    pub value: Box<Expression<'s>>,
}

/// A struct declaration statement.
///
/// Grammar: `struct` [Identifier] [StructDeclaration]
#[derive(Debug, Clone)]
pub struct StructDeclarationStatement<'s> {
    pub struct_: &'s Token<'s>,
    pub name: Identifier<'s>,
    pub declaration: StructDeclaration<'s>,
}

/// A type definition statement.
///
/// Grammar: `typedef` [Identifier] [Type]
#[derive(Debug, Clone)]
pub struct TypedefStatement<'s> {
    pub typedef: &'s Token<'s>,
    pub name: Identifier<'s>,
    pub ty: Type<'s>,
}

/// A global declaration statement.
///
/// Grammar: `global` [GlobalDeclaration]
#[derive(Debug, Clone)]
pub struct GlobalStatement<'s> {
    pub global: &'s Token<'s>,
    pub declaration: GlobalDeclaration<'s>,
}

/// A `globalize_all_functions` statement.
///
/// Grammar: `globalize_all_functions`
#[derive(Debug, Clone)]
pub struct GlobalizeAllFunctionsStatement<'s> {
    pub globalize_all_functions: &'s Token<'s>,
}

/// An `untyped` statement.
///
/// Grammar: `untyped`
#[derive(Debug, Clone)]
pub struct UntypedStatement<'s> {
    pub untyped: &'s Token<'s>,
}
