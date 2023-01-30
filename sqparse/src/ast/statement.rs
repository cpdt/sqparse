use crate::ast::{
    ClassDeclaration, Expression, FunctionDeclaration, Identifier, SeparatedList1,
    StructDeclaration, Type, VarInitializer,
};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Statement<'s> {
    pub ty: StatementType<'s>,

    pub end_token: Option<&'s Token<'s>>,
}

#[derive(Debug, Clone)]
pub enum StatementType<'s> {
    Empty,
    Blank(&'s Token<'s>),
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
    VarDeclaration(VarDeclarationStatement<'s>),
    FunctionDeclaration(FunctionDeclarationStatement<'s>),
    ClassDeclaration(ClassDeclarationStatement<'s>),
    TryCatch(TryCatchStatement<'s>),
    Throw(ThrowStatement<'s>),
    Const(ConstStatement<'s>),
    Enum(EnumStatement<'s>),
    Expression(Box<Expression<'s>>),

    // _re additions
    Thread(ThreadStatement<'s>),
    DelayThread(DelayThreadStatement<'s>),
    WaitThread(WaitThreadStatement<'s>),
    Wait(WaitStatement<'s>),
    StructDeclaration(StructDeclarationStatement<'s>),
    TypeDeclaration(TypeDeclarationStatement<'s>),
    Global(GlobalStatement<'s>),
    GlobalizeAllFunctions(GlobalizeAllFunctionsStatement<'s>),
    Untyped(UntypedStatement<'s>),
}

//

#[derive(Debug, Clone)]
pub struct BlockStatement<'s> {
    pub body: Vec<Statement<'s>>,

    pub open_token: &'s Token<'s>,
    pub close_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct IfStatement<'s> {
    pub condition: Box<Expression<'s>>,
    pub body: Box<Statement<'s>>,
    pub else_: Option<IfElse<'s>>,

    pub if_token: &'s Token<'s>,
    pub open_condition_token: &'s Token<'s>,
    pub close_condition_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct WhileStatement<'s> {
    pub condition: Box<Expression<'s>>,
    pub body: Box<Statement<'s>>,

    pub while_token: &'s Token<'s>,
    pub open_condition_token: &'s Token<'s>,
    pub close_condition_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct DoWhileStatement<'s> {
    pub body: Box<Statement<'s>>,
    pub condition: Box<Expression<'s>>,

    pub do_token: &'s Token<'s>,
    pub while_token: &'s Token<'s>,
    pub open_condition_token: &'s Token<'s>,
    pub close_condition_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct SwitchStatement<'s> {
    pub condition: Box<Expression<'s>>,
    pub cases: Vec<SwitchCase<'s>>,

    pub switch_token: &'s Token<'s>,
    pub open_condition_token: &'s Token<'s>,
    pub close_condition_token: &'s Token<'s>,
    pub open_cases_token: &'s Token<'s>,
    pub close_cases_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct ForStatement<'s> {
    pub initializer: Option<ExpressionOrDeclaration<'s>>,
    pub condition: Option<Box<Expression<'s>>>,
    pub increment: Option<Box<Expression<'s>>>,
    pub body: Box<Statement<'s>>,

    pub for_token: &'s Token<'s>,
    pub open_header_token: &'s Token<'s>,
    pub close_header_token: &'s Token<'s>,
    pub first_separator_token: &'s Token<'s>,
    pub second_separator_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct ForeachStatement<'s> {
    pub index: Option<ForeachIndex<'s>>,
    pub value_type: Option<Type<'s>>,
    pub value_name: Identifier<'s>,
    pub array: Box<Expression<'s>>,
    pub body: Box<Statement<'s>>,

    pub foreach_token: &'s Token<'s>,
    pub open_header_token: &'s Token<'s>,
    pub close_header_token: &'s Token<'s>,
    pub separator_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct BreakStatement<'s> {
    pub token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct ContinueStatement<'s> {
    pub token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct ReturnStatement<'s> {
    pub value: Option<Box<Expression<'s>>>,
    pub return_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct YieldStatement<'s> {
    pub value: Option<Box<Expression<'s>>>,
    pub yield_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct VarDeclarationStatement<'s> {
    pub var_type: Type<'s>,
    pub declarations: SeparatedList1<'s, VarDeclaration<'s>>,

    pub trailing_separator_token: Option<&'s Token<'s>>,
}

#[derive(Debug, Clone)]
pub struct FunctionDeclarationStatement<'s> {
    pub return_type: Option<Type<'s>>,
    pub name: SeparatedList1<'s, Identifier<'s>>,
    pub function: FunctionDeclaration<'s>,

    pub function_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct ClassDeclarationStatement<'s> {
    pub name: Box<Expression<'s>>,
    pub class: ClassDeclaration<'s>,

    pub class_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct TryCatchStatement<'s> {
    pub body: Box<Statement<'s>>,
    pub catch_name: Identifier<'s>,
    pub catch_body: Box<Statement<'s>>,

    pub try_token: &'s Token<'s>,
    pub catch_token: &'s Token<'s>,
    pub open_catch_name_token: &'s Token<'s>,
    pub close_catch_name_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct ThrowStatement<'s> {
    pub value: Box<Expression<'s>>,

    pub throw_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct ConstStatement<'s> {
    pub const_type: Option<Type<'s>>,
    pub name: Identifier<'s>,
    pub value: Box<Expression<'s>>,

    pub const_token: &'s Token<'s>,
    pub separator_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct EnumStatement<'s> {
    pub name: Identifier<'s>,
    pub entries: Vec<EnumEntry<'s>>,

    pub enum_token: &'s Token<'s>,
    pub open_entries_token: &'s Token<'s>,
    pub close_entries_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct ThreadStatement<'s> {
    pub value: Box<Expression<'s>>,

    pub thread_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct DelayThreadStatement<'s> {
    pub duration: Box<Expression<'s>>,
    pub value: Box<Expression<'s>>,

    pub delay_thread_token: &'s Token<'s>,
    pub open_duration_token: &'s Token<'s>,
    pub close_duration_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct WaitThreadStatement<'s> {
    pub value: Box<Expression<'s>>,

    pub wait_thread_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct WaitStatement<'s> {
    pub value: Box<Expression<'s>>,

    pub wait_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct StructDeclarationStatement<'s> {
    pub name: Identifier<'s>,
    pub declaration: StructDeclaration<'s>,

    pub struct_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct TypeDeclarationStatement<'s> {
    pub name: Identifier<'s>,
    pub base_type: Type<'s>,

    pub typedef_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct GlobalStatement<'s> {
    pub ty: GlobalType<'s>,

    pub global_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct GlobalizeAllFunctionsStatement<'s> {
    pub token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct UntypedStatement<'s> {
    pub token: &'s Token<'s>,
}

//

#[derive(Debug, Clone)]
pub struct IfElse<'s> {
    pub statement: Box<Statement<'s>>,

    pub else_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct SwitchCase<'s> {
    pub condition: SwitchCaseCondition<'s>,
    pub body: Vec<Statement<'s>>,

    pub end_condition_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub enum ExpressionOrDeclaration<'s> {
    Expression(Box<Expression<'s>>),
    Declaration(VarDeclarationStatement<'s>),
}

#[derive(Debug, Clone)]
pub struct ForeachIndex<'s> {
    pub name_type: Option<Type<'s>>,
    pub name: Identifier<'s>,

    pub separator_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct VarDeclaration<'s> {
    pub name: Identifier<'s>,
    pub initializer: Option<VarInitializer<'s>>,
}

#[derive(Debug, Clone)]
pub struct EnumEntry<'s> {
    pub name: Identifier<'s>,
    pub initializer: Option<VarInitializer<'s>>,

    pub separator_token: Option<&'s Token<'s>>,
}

#[derive(Debug, Clone)]
pub enum GlobalType<'s> {
    Function {
        name: SeparatedList1<'s, Identifier<'s>>,

        function_token: &'s Token<'s>,
    },
    VarDeclaration(VarDeclarationStatement<'s>),
    Const(ConstStatement<'s>),
    Enum(EnumStatement<'s>),
    Class(ClassDeclarationStatement<'s>),
    Struct(StructDeclarationStatement<'s>),
    Type(TypeDeclarationStatement<'s>),
}

//

#[derive(Debug, Clone)]
pub enum SwitchCaseCondition<'s> {
    Default {
        default_token: &'s Token<'s>,
    },
    Expr {
        expr: Box<Expression<'s>>,
        case_token: &'s Token<'s>,
    },
}
