/// Context information that can be attached to a [`ParseError`].
///
/// Essentially, if an error has a context, it means that the parser knows that the error is inside
/// some specific syntactical construct.
///
/// This is intended to hint at the state of the parser to make syntax errors and parser debugging a
/// bit easier.
///
/// Implements [`std::fmt::Display`] to write a useful description of the context.
///
/// [`ParseError`]: crate::ParseError
#[derive(Debug, Clone, Copy)]
pub enum ContextType {
    ParensExpression,
    ClassDeclaration,
    StructDeclaration,
    IfStatement,
    ElseStatement,
    WhileStatement,
    DoWhileStatement,
    SwitchStatement,
    ForStatement,
    ForeachStatement,
    ReturnStatement,
    YieldStatement,
    VarDeclarationStatement,
    FunctionDeclarationStatement,
    ClassStatement,
    TryCatchStatement,
    ThrowStatement,
    ConstStatement,
    EnumStatement,
    ThreadStatement,
    DelayThreadStatement,
    WaitThreadStatement,
    WaitStatement,
    StructStatement,
    TypedefStatement,
    GlobalStatement,
    FunctionRefType,
    StructType,
    ArrayType,
    GenericType,

    ExpressionRightHandSide,
    RootVarExpression,
    DelegateExpression,
    VectorExpression,
    TernaryExpression,
    IndexExpression,
    CallExpression,
    CommaExpression,
    TableExpression,
    ClassExpression,
    ArrayExpression,
    ExpectExpression,
    FunctionExpression,
    PropertyTableSlot,
    ComputedPropertyTableSlot,
    JsonPropertyTableSlot,
    FunctionTableSlot,
    ClassExtends,
    PropertyClassMember,
    ComputedPropertyClassMember,
    ConstructorClassMember,
    FunctionClassMember,
    FunctionDeclarationArgs,
    FunctionDeclarationEnvironment,
    FunctionDeclarationCaptures,
    VarInitializer,
    FunctionGlobal,
    SwitchCaseCondition,
    BlockStatement,
}

impl ContextType {
    /// Returns if the context is "useful".
    ///
    /// This is intended to allow prioritising some context types over others.
    ///
    /// For example, in this code:
    /// ```text
    /// if (a > b) {
    ///     invalid code
    /// }
    /// ```
    ///
    /// There are two possible contexts here, the [`IfStatement`] and the [`BlockStatement`] inside.
    /// Normally an error contains the inner-most context, which in this case would be the
    /// [`BlockStatement`]. But in this case the [`IfStatement`] is far more useful.
    ///
    /// [`BlockStatement`] is marked as _not useful_, meaning it will be overridden if other
    /// useful contextual information is available.
    ///
    /// [`IfStatement`]: ContextType::IfStatement
    /// [`BlockStatement`]: ContextType::BlockStatement
    pub fn is_useful(self) -> bool {
        !matches!(self, ContextType::BlockStatement)
    }
}

impl std::fmt::Display for ContextType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextType::ParensExpression => write!(f, "expression"),
            ContextType::ClassDeclaration => write!(f, "class"),
            ContextType::StructDeclaration => write!(f, "struct"),
            ContextType::IfStatement => write!(f, "`if` statement"),
            ContextType::ElseStatement => write!(f, "`else` statement"),
            ContextType::WhileStatement => write!(f, "`while` statement"),
            ContextType::DoWhileStatement => write!(f, "`do while` statement"),
            ContextType::SwitchStatement => write!(f, "`switch` statement"),
            ContextType::ForStatement => write!(f, "`for` statement"),
            ContextType::ForeachStatement => write!(f, "`foreach` statement"),
            ContextType::ReturnStatement => write!(f, "`return` statement"),
            ContextType::YieldStatement => write!(f, "`yield` statement"),
            ContextType::VarDeclarationStatement => write!(f, "`var` statement"),
            ContextType::FunctionDeclarationStatement => write!(f, "`function` statement"),
            ContextType::ClassStatement => write!(f, "`class` statement"),
            ContextType::TryCatchStatement => write!(f, "`try` statement"),
            ContextType::ThrowStatement => write!(f, "`throw` statement"),
            ContextType::ConstStatement => write!(f, "`const` statement"),
            ContextType::EnumStatement => write!(f, " `enum` statement"),
            ContextType::ThreadStatement => write!(f, "`thread` statement"),
            ContextType::DelayThreadStatement => write!(f, "`delaythread` statement"),
            ContextType::WaitThreadStatement => write!(f, "`waitthread` statement"),
            ContextType::WaitStatement => write!(f, "`wait` statement"),
            ContextType::StructStatement => write!(f, "`struct` statement"),
            ContextType::TypedefStatement => write!(f, "`typedef` statement"),
            ContextType::GlobalStatement => write!(f, "`global` statement"),
            ContextType::FunctionRefType => write!(f, "`functionref` type"),
            ContextType::StructType => write!(f, "`struct` type"),
            ContextType::ArrayType => write!(f, "array type"),
            ContextType::GenericType => write!(f, "generic type"),

            ContextType::RootVarExpression => write!(f, "`::` expression"),
            ContextType::DelegateExpression => write!(f, "`delegate` expression"),
            ContextType::VectorExpression => write!(f, "vector expression"),
            ContextType::TernaryExpression => write!(f, "ternary expression"),
            ContextType::ExpressionRightHandSide => write!(f, "right hand side"),
            ContextType::IndexExpression => write!(f, "index expression"),
            ContextType::CallExpression => write!(f, "call expression"),
            ContextType::CommaExpression => write!(f, "`,` expression"),
            ContextType::TableExpression => write!(f, "table expression"),
            ContextType::ClassExpression => write!(f, "`class` expression"),
            ContextType::ArrayExpression => write!(f, "array expression"),
            ContextType::ExpectExpression => write!(f, "`expect` expression"),
            ContextType::FunctionExpression => write!(f, "`function` expression"),
            ContextType::PropertyTableSlot
            | ContextType::ComputedPropertyTableSlot
            | ContextType::JsonPropertyTableSlot => write!(f, "table property"),
            ContextType::FunctionTableSlot => write!(f, "table function"),
            ContextType::ClassExtends => write!(f, "class declaration"),
            ContextType::PropertyClassMember | ContextType::ComputedPropertyClassMember => {
                write!(f, "class property")
            }
            ContextType::ConstructorClassMember => write!(f, "class constructor"),
            ContextType::FunctionClassMember => write!(f, "class method"),
            ContextType::FunctionDeclarationArgs => write!(f, "function argument list"),
            ContextType::FunctionDeclarationEnvironment => write!(f, "function environment"),
            ContextType::FunctionDeclarationCaptures => write!(f, "function capture list"),
            ContextType::VarInitializer => write!(f, "initializer"),
            ContextType::FunctionGlobal => write!(f, "`function` global"),
            ContextType::SwitchCaseCondition => write!(f, "`switch` case"),
            ContextType::BlockStatement => write!(f, "block statement"),
        }
    }
}
