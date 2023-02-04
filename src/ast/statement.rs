use crate::ast::{
    ClassDefinition, EnumEntry, Expression, ForDefinition, ForeachIndex, FunctionDefinition,
    GlobalDefinition, Identifier, IfStatementType, SeparatedList1, SeparatedListTrailing1,
    StructDefinition, SwitchCase, Type, VarDefinition, VarInitializer,
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
    pub semicolon: Option<&'s Token<'s>>,
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
    Foreach(ForeachStatement<'s>),
    Break(BreakStatement<'s>),
    Continue(ContinueStatement<'s>),
    Return(ReturnStatement<'s>),
    Yield(YieldStatement<'s>),
    VarDefinition(VarDefinitionStatement<'s>),
    ConstructorDefinition(ConstructorDefinitionStatement<'s>),
    FunctionDefinition(FunctionDefinitionStatement<'s>),
    ClassDefinition(ClassDefinitionStatement<'s>),
    TryCatch(TryCatchStatement<'s>),
    Throw(ThrowStatement<'s>),
    Const(ConstDefinitionStatement<'s>),
    EnumDefinition(EnumDefinitionStatement<'s>),
    Expression(ExpressionStatement<'s>),

    // _re additions
    Thread(ThreadStatement<'s>),
    DelayThread(DelayThreadStatement<'s>),
    WaitThread(WaitThreadStatement<'s>),
    Wait(WaitStatement<'s>),
    StructDefinition(StructDefinitionStatement<'s>),
    TypeDefinition(TypeDefinitionStatement<'s>),
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
/// Grammar: `if` `(` [Expression] `)` [IfStatementType]
#[derive(Debug, Clone)]
pub struct IfStatement<'s> {
    pub if_: &'s Token<'s>,
    pub open: &'s Token<'s>,
    pub condition: Box<Expression<'s>>,
    pub close: &'s Token<'s>,
    pub ty: IfStatementType<'s>,
}

/// A `while` statement.
///
/// Grammar: `while` `(` [Expression] `)` [StatementType]
#[derive(Debug, Clone)]
pub struct WhileStatement<'s> {
    pub while_: &'s Token<'s>,
    pub open: &'s Token<'s>,
    pub condition: Box<Expression<'s>>,
    pub close: &'s Token<'s>,
    pub body: Box<StatementType<'s>>,
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
/// Grammar: `for` `(` [ForDefinition]? `;` [Expression]? `;` [Expression]? `)` [StatementType]
#[derive(Debug, Clone)]
pub struct ForStatement<'s> {
    pub for_: &'s Token<'s>,
    pub open: &'s Token<'s>,
    pub initializer: Option<ForDefinition<'s>>,
    pub semicolon_1: &'s Token<'s>,
    pub condition: Option<Box<Expression<'s>>>,
    pub semicolon_2: &'s Token<'s>,
    pub increment: Option<Box<Expression<'s>>>,
    pub close: &'s Token<'s>,
    pub body: Box<StatementType<'s>>,
}

/// A `foreach` loop statement.
///
/// Grammar: `foreach` `(` [ForeachIndex]? [Type]? [Identifier] `in` [Expression] `)` [StatementType]
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
    pub body: Box<StatementType<'s>>,
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

/// A variable definition statement.
///
/// Grammar: [Type] [SeparatedListTrailing1]<[VarDefinition] `,`>
#[derive(Debug, Clone)]
pub struct VarDefinitionStatement<'s> {
    pub type_: Type<'s>,
    pub definitions: SeparatedListTrailing1<'s, VarDefinition<'s>>,
}

/// An out-of-band constructor definition statement.
///
/// Grammar: `function` ([Identifier] `::`)+ `constructor` [FunctionDefinition]
#[derive(Debug, Clone)]
pub struct ConstructorDefinitionStatement<'s> {
    pub function: &'s Token<'s>,
    pub namespaces: Vec<(Identifier<'s>, &'s Token<'s>)>,
    pub last_name: Identifier<'s>,
    pub last_namespace: &'s Token<'s>,
    pub constructor: &'s Token<'s>,
    pub definition: FunctionDefinition<'s>,
}

/// A function definition statement.
///
/// Grammar: [Type]? `function` [SeparatedList1]<[Identifier] `::`> [FunctionDefinition]
#[derive(Debug, Clone)]
pub struct FunctionDefinitionStatement<'s> {
    pub return_type: Option<Type<'s>>,
    pub function: &'s Token<'s>,
    pub name: SeparatedList1<'s, Identifier<'s>>,
    pub definition: FunctionDefinition<'s>,
}

/// A class definition statement.
///
/// Grammar: `class` [Expression] [ClassDefinition]
#[derive(Debug, Clone)]
pub struct ClassDefinitionStatement<'s> {
    pub class: &'s Token<'s>,
    pub name: Box<Expression<'s>>,
    pub definition: ClassDefinition<'s>,
}

/// A `try-catch` statement.
///
/// Grammar: `try` [Statement] `catch` `(` [Identifier] `)` [StatementType]
#[derive(Debug, Clone)]
pub struct TryCatchStatement<'s> {
    pub try_: &'s Token<'s>,
    pub body: Box<Statement<'s>>,
    pub catch: &'s Token<'s>,
    pub open: &'s Token<'s>,
    pub catch_name: Identifier<'s>,
    pub close: &'s Token<'s>,
    pub catch_body: Box<StatementType<'s>>,
}

/// A `throw` statement.
///
/// Grammar: `throw` [Expression]
#[derive(Debug, Clone)]
pub struct ThrowStatement<'s> {
    pub throw: &'s Token<'s>,
    pub value: Box<Expression<'s>>,
}

/// A `const` definition statement.
///
/// Grammar: `const` [Type]? [Identifier] [VarInitializer]
#[derive(Debug, Clone)]
pub struct ConstDefinitionStatement<'s> {
    pub const_: &'s Token<'s>,
    pub const_type: Option<Type<'s>>,
    pub name: Identifier<'s>,
    pub initializer: VarInitializer<'s>,
}

/// An `enum` definition statement.
///
/// Grammar: `enum` [Identifier] `{` [EnumEntry]* `}`
#[derive(Debug, Clone)]
pub struct EnumDefinitionStatement<'s> {
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

/// A struct definition statement.
///
/// Grammar: `struct` [Identifier] [StructDefinition]
#[derive(Debug, Clone)]
pub struct StructDefinitionStatement<'s> {
    pub struct_: &'s Token<'s>,
    pub name: Identifier<'s>,
    pub definition: StructDefinition<'s>,
}

/// A type definition statement.
///
/// Grammar: `typedef` [Identifier] [Type]
#[derive(Debug, Clone)]
pub struct TypeDefinitionStatement<'s> {
    pub typedef: &'s Token<'s>,
    pub name: Identifier<'s>,
    pub type_: Type<'s>,
}

/// A global definition statement.
///
/// Grammar: `global` [GlobalDefinition]
#[derive(Debug, Clone)]
pub struct GlobalStatement<'s> {
    pub global: &'s Token<'s>,
    pub definition: GlobalDefinition<'s>,
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
