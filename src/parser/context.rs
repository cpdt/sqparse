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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextType {
    /// A span of something. This should generally be replaced with a more specific context.
    Span,

    /// An expression with a value and an operator.
    ///
    /// # Example
    /// ```text
    /// (1 + 5) * 3--
    /// ^^^^^^^   ^^^ expression
    /// ^^^^^^^^^^^^^ expression
    /// ```
    Expression,

    /// Literal defining a table.
    ///
    /// # Example
    /// ```text
    /// local ages = SumAges({ Charlie = 28, Maeve = 23 })
    ///                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ table literal
    /// ```
    TableLiteral,

    /// Literal defining an array.
    ///
    /// # Example
    /// ```text
    /// local cities = ["Adelaide", "Sydney"].join(", ")
    ///                ^^^^^^^^^^^^^^^^^^^^^^ array literal
    /// ```
    ArrayLiteral,

    /// Literal defining a 3D vector.
    ///
    /// # Example
    /// ```text
    /// player.LookAt(< -5.2, 0.0, 10.0 >)
    ///               ^^^^^^^^^^^^^^^^^^^ vector literal
    /// ```
    VectorLiteral,

    /// Literal defining an anonymous function.
    ///
    /// # Example
    /// ```text
    /// local writeVal = function() { }
    ///                  ^^^^^^^^^^^^^^ function literal
    /// ```
    FunctionLiteral,

    /// Literal defining a lambda function.
    ///
    /// # Example
    /// ```text
    /// local myexp = @(a,b) a + b
    ///               ^^^^^^^^^^^^ lambda literal
    /// ```
    LambdaLiteral,

    /// Literal defining an anonymous class.
    ///
    /// # Example
    /// ```text
    /// local ageManager = class { Ages = ages }
    ///                    ^^^^^^^^^^^^^^^^^^^^^ class literal
    /// ```
    ClassLiteral,

    /// List of arguments in a function call.
    ///
    /// # Example
    /// ```text
    /// player.SayTo(friend, "Hello there!")
    ///              ^^^^^^^^^^^^^^^^^^^^^^ call arguments
    /// ```
    CallArgumentList,

    /// A statement in a program or block.
    ///
    /// # Example
    /// ```text
    /// local name = "squirrel"
    /// ^^^^^^^^^^^^^^^^^^^^^^^ statement
    /// ```
    Statement,

    /// A block statement, containing multiple sub-statements.
    ///
    /// # Example
    /// ```text
    /// local count = 6;
    ///   {
    ///  _^
    /// |     log("hello");
    /// | }
    /// |_^ block statement
    /// log("world");
    /// ```
    BlockStatement,

    /// All of an `if` statement.
    ///
    /// # Example
    /// ```text
    ///   local isTimedOut = player.IsTimedOut()
    ///   if (isTimedOut && player.IsAlive()) {
    ///  _^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// |     player.Kill()
    /// | }
    /// |_^ if statement
    /// ```text
    IfStatement,

    /// The condition in an `if` statement.
    ///
    /// # Example
    /// ```text
    /// if ( IsLoggedIn() ) LogOut()
    ///    ^^^^^^^^^^^^^^^^ if statement condition
    /// ```
    IfStatementCondition,

    /// All of a `while` statement.
    ///
    /// # Example
    /// ```text
    ///   while (!IsDay()) {
    ///  _^^^^^^^^^^^^^^^^^^
    /// |     Sleep();
    /// | }
    /// |_^ while statement
    ///   WakeUp();
    /// ```
    WhileStatement,

    /// The condition of a `while` statement.
    ///
    /// # Example
    /// ```text
    /// while (!isDone) Continue()
    ///       ^^^^^^^^^ while statement condition
    /// ```
    WhileStatementCondition,

    /// All of a `do while` statement.
    ///
    /// # Example
    /// ```text
    ///   do {
    ///  _^^^^
    /// |     Shoot();
    /// | } while ( IsButtonDown() )
    /// |_^^^^^^^^^^^^^^^^^^^^^^^^^^ do while statement
    /// ```
    DoWhileStatement,

    /// The condition of a `do while` statement.
    ///
    /// # Example
    /// ```text
    /// do {
    ///     train()
    /// } while (fitness < 10)
    ///         ^^^^^^^^^^^^^^ do while statement condition
    ///
    /// ```
    DoWhileStatementCondition,

    /// All of a `switch` statement.
    ///
    /// # Example
    /// ```text
    ///   switch (state) {
    ///  _^^^^^^^^^^^^^^^^
    /// |     case "playing": play(); break;
    /// |     cause "paused": pause(); break;
    /// | }
    /// |_^ switch statement
    /// ```
    SwitchStatement,

    /// The condition of a `switch` statement.
    ///
    /// # Example
    /// ```text
    /// switch ( GameMode() ) {
    ///        ^^^^^^^^^^^^^^ switch statement condition
    ///     case "slayer":
    ///         return;
    ///     case "creative":
    ///         SetHealth(100);
    ///         break;
    /// }
    /// ```
    SwitchStatementCondition,

    /// All of a `for` statement.
    ///
    /// # Example
    /// ```text
    ///   for (local i = 0; i < 10; i++) {
    ///  _^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// |     println(i)
    /// | }
    /// |_^ for statement
    /// ```
    ForStatement,

    /// The condition of a `for` statement.
    ///
    /// # Example
    /// ```text
    /// for (local i = 0; i < players.len; i++) {
    ///     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ for statement condition
    ///     players[i].say("hello")
    /// }
    /// ```
    ForStatementCondition,

    /// All of a `foreach` statement.
    ///
    /// # Example
    /// ```text
    ///   foreach (i, video in videos) {
    ///  _^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// |     println("playing " + i)
    /// |     play(video)
    /// | }
    /// |_^
    /// ```
    ForeachStatement,

    /// The condition of a `foreach` statement.
    ///
    /// # Example
    /// ```text
    /// foreach (Map map in maps) {
    ///         ^^^^^^^^^^^^^^^^^ foreach statement condition
    ///     map.load()
    ///     println("loaded map: " + map.name)
    /// }
    /// ```
    ForeachStatementCondition,

    /// All of a `try catch` statement.
    ///
    /// # Example
    /// ```text
    ///   try {
    ///  _^^^^^
    /// |     FetchData()
    /// | } catch (error) {
    /// |     println("could not fetch data: " + error)
    /// | }
    /// |_^ try catch statement
    /// ```
    TryCatchStatement,

    /// The catch binding in a `try catch` statement.
    ///
    /// # Example
    /// ```text
    /// try {
    ///     money += withdraw(500)
    /// } catch (error) {
    ///         ^^^^^^^ try catch statement catch name
    ///     println(error)
    /// }
    /// ```
    TryCatchStatementCatchName,

    /// A `return` statement in a function.
    ///
    /// # Example
    /// ```text
    /// return rnd(0, 10)
    /// ^^^^^^^^^^^^^^^^^ return statement
    /// ```
    ReturnStatement,

    /// A `yield` statement in a generator function.
    ///
    /// # Example
    /// ```text
    /// foreach (player in players) {
    ///     yield player.health
    ///     ^^^^^^^^^^^^^^^^^^^ yield statement
    /// }
    /// ```
    YieldStatement,

    /// A `throw` statement.
    ///
    /// # Example
    /// ```text
    /// if ( !request.success ) {
    ///     throw "Request Failed"
    ///     ^^^^^^^^^^^^^^^^^^^^^^ throw statement
    /// }
    /// ```
    ThrowStatement,

    /// A `thread` statement.
    ///
    /// # Example
    /// ```text
    /// thread PostmatchScreen()
    /// ^^^^^^^^^^^^^^^^^^^^^^^^ thread statement
    /// ```
    ThreadStatement,

    /// A `delaythread` statement.
    ///
    /// # Example
    /// ```text
    /// delaythread(10) alert("times up!")
    /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ delay thread statement
    /// ```
    DelayThreadStatement,

    /// A `waitthread` statement.
    ///
    /// # Example
    /// ```text
    /// waitthread readAllFiles()
    /// ^^^^^^^^^^^^^^^^^^^^^^^^^ wait thread statement
    /// ```
    WaitThreadStatement,

    /// A `waitthreadsolo` statement.
    ///
    /// # Example
    /// ```text
    /// waitthreadsolo WaitForPlayerExitButtonPressed( player )
    /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ wait thread solo statement
    /// ```
    WaitThreadSoloStatement,

    /// A `wait` statement.
    ///
    /// # Example
    /// ```text
    /// println("3")
    /// wait 1
    /// ^^^^^^ wait statement
    /// println("2")
    /// ```
    WaitStatement,

    /// A `global` statement.
    ///
    /// # Example
    /// ```text
    /// global MaxThreads = 10
    /// ^^^^^^^^^^^^^^^^^^^^^^ global statement
    /// ```
    GlobalStatement,

    /// Class definition.
    ///
    /// # Example
    /// ```text
    ///   class Person {
    ///  _^^^^^^^^^^^^^^
    /// |     alive = true
    /// |     constructor(name) {
    /// |         this.name <- name
    /// |     }
    /// | }
    /// |_^ class definition
    /// ```
    ClassDefinition,

    /// Enum definition.
    ///
    /// # Example
    /// ```text
    ///   enum GameMode {
    ///  _^^^^^^^^^^^^^^^
    /// |     SURVIVAL,
    /// |     CREATIVE,
    /// |     ADVENTURE,
    /// | }
    /// |_^ enum definition
    /// ```
    EnumDefinition,

    /// Function definition.
    ///
    /// # Example
    /// ```text
    ///   void function Damage( entity player ) {
    ///  _^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// |     player.health -= 5
    /// | }
    /// |_^ function definition
    ///   Damage( me )
    /// ```
    FunctionDefinition,

    /// Constant value definition.
    ///
    /// # Example
    /// ```text
    /// const MaxHealth = 10;
    /// ^^^^^^^^^^^^^^^^^^^^^ const definition
    /// player.health = MaxHealth
    /// ```
    ConstDefinition,

    /// Variable definition.
    ///
    /// # Example
    /// ```text
    /// local cities = ["Adelaide", "Sydney"]
    /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ variable definition
    /// ```
    VarDefinition,

    /// Struct definition.
    ///
    /// # Example
    /// ```text
    ///   struct Waypoint {
    ///  _^^^^^^^^^^^^^^^^^
    /// |     float x,
    /// |     float y,
    /// |     string name
    /// | }
    /// |_^ struct definition
    /// ```
    StructDefinition,

    /// Type definition.
    ///
    /// # Example
    /// ```text
    /// typedef PlayerMap table<string, player>
    /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type definition
    /// ```
    TypeDefinition,

    /// Property in a class, table, or struct.
    ///
    /// # Example
    /// ```text
    /// class Player {
    ///     isPlaying = true
    ///     ^^^^^^^^^^^^^^^^ class property
    /// }
    /// ```
    Property,

    /// Constructor in a class or table.
    ///
    /// # Example
    /// ```text
    ///   class FunMachine() {
    ///       constructor() {
    ///  _____^^^^^^^^^^^^^^^
    /// |         this.funLevel <- 11
    /// |     }
    /// |_____^ class constructor
    ///   }
    /// ```
    Constructor,

    /// Method in a class or table.
    ///
    /// # Example
    /// ```text
    ///   class FooCalculator {
    ///       int function calculateFoo() {
    ///  _____^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// |         return 4
    /// |     }
    /// | }
    /// |_^ class method
    /// ```
    Method,

    /// Function definition's parameter list.
    ///
    /// # Example
    /// ```text
    /// function foobar(int a, string b = "no", ...) {
    ///                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ function param list
    /// }
    /// ```
    FunctionParamList,

    /// Function environment definition.
    ///
    /// # Example
    /// ```text
    /// local person = { age = 32 }
    /// function printPersonDetails[person]() {
    ///                            ^^^^^^^^ function environment
    ///     println(this.age)
    /// }
    /// ```
    FunctionEnvironment,

    /// Function free variable/capture list.
    ///
    /// # Example
    /// ```text
    /// filterFunc = function(val) : (max) {
    ///                              ^^^^^ function capture list
    ///     return val <= max
    /// }
    /// ```
    FunctionCaptureList,

    /// A type with an optional number of type modifiers.
    ///
    /// # Example
    /// ```text
    /// table<string, person&> ornull
    /// ^^^^^ ^^^^^^  ^^^^^^          type
    ///               ^^^^^^^         type
    /// ^^^^^^^^^^^^^^^^^^^^^^        type
    /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type
    /// ```
    Type,

    /// List of arguments in a generic type.
    ///
    /// # Example
    /// ```text
    /// table<string, country>
    ///      ^^^^^^^^^^^^^^^^^ generic argument list
    /// ```
    GenericArgumentList,
}

impl std::fmt::Display for ContextType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextType::Span => write!(f, "span"),
            ContextType::Expression => write!(f, "expression"),
            ContextType::TableLiteral => write!(f, "table literal"),
            ContextType::ArrayLiteral => write!(f, "array literal"),
            ContextType::VectorLiteral => write!(f, "vector literal"),
            ContextType::FunctionLiteral => write!(f, "function literal"),
            ContextType::LambdaLiteral => write!(f, "lambda literal"),
            ContextType::ClassLiteral => write!(f, "class literal"),
            ContextType::CallArgumentList => write!(f, "argument list"),
            ContextType::Statement => write!(f, "statement"),
            ContextType::BlockStatement => write!(f, "block statement"),
            ContextType::IfStatement => write!(f, "`if` statement"),
            ContextType::IfStatementCondition => write!(f, "`if` statement condition"),
            ContextType::WhileStatement => write!(f, "`while` loop"),
            ContextType::WhileStatementCondition => write!(f, "`while` loop condition"),
            ContextType::DoWhileStatement => write!(f, "`do while` loop"),
            ContextType::DoWhileStatementCondition => write!(f, "`do while` loop condition"),
            ContextType::SwitchStatement => write!(f, "`switch` statement"),
            ContextType::SwitchStatementCondition => write!(f, "`switch` statement condition"),
            ContextType::ForStatement => write!(f, "`for` loop"),
            ContextType::ForStatementCondition => write!(f, "`for` loop condition"),
            ContextType::ForeachStatement => write!(f, "`foreach` loop"),
            ContextType::ForeachStatementCondition => write!(f, "`foreach` loop condition"),
            ContextType::TryCatchStatement => write!(f, "`try` statement"),
            ContextType::TryCatchStatementCatchName => write!(f, "`try catch` name"),
            ContextType::ReturnStatement => write!(f, "`return` statement"),
            ContextType::YieldStatement => write!(f, "`yield` statement"),
            ContextType::ThrowStatement => write!(f, "`throw` statement"),
            ContextType::ThreadStatement => write!(f, "`thread` statement"),
            ContextType::DelayThreadStatement => write!(f, "`delaythread` statement"),
            ContextType::WaitThreadStatement => write!(f, "`waitthread` statement"),
            ContextType::WaitThreadSoloStatement => write!(f, "`waitthreadsolo` statement"),
            ContextType::WaitStatement => write!(f, "`wait` statement"),
            ContextType::GlobalStatement => write!(f, "`global` statement"),
            ContextType::ClassDefinition => write!(f, "`class` definition"),
            ContextType::EnumDefinition => write!(f, "`enum` definition"),
            ContextType::FunctionDefinition => write!(f, "`function` definition"),
            ContextType::ConstDefinition => write!(f, "`const` definition"),
            ContextType::VarDefinition => write!(f, "variable definition"),
            ContextType::StructDefinition => write!(f, "`struct` definition"),
            ContextType::TypeDefinition => write!(f, "type definition"),
            ContextType::Property => write!(f, "property"),
            ContextType::Constructor => write!(f, "constructor"),
            ContextType::Method => write!(f, "method"),
            ContextType::FunctionParamList => write!(f, "parameter list"),
            ContextType::FunctionEnvironment => write!(f, "function environment"),
            ContextType::FunctionCaptureList => write!(f, "function capture list"),
            ContextType::Type => write!(f, "type"),
            ContextType::GenericArgumentList => write!(f, "generic type list"),
        }
    }
}
