//! Token types emitted by the lexer.
//!
//! A [`Token`] can be a:
//!  - [`TerminalToken`]: an atomic symbol or reserved identifier.
//!  - [`LiteralToken`]: an integer, float, char or string literal.
//!  - [`Identifier`]: an identifier, for naming things.
//!
//! Tokens are also fully source-preserving. Each token maintains its range in the input string,
//! as well as the comments and newlines after the token.
//!
//! [`Identifier`]: TokenType::Identifier

use crate::Flavor;
use std::ops::Range;

macro_rules! _terminal_matches {
    ($ask_flavor:ident) => {
        true
    };
    ($ask_flavor:ident $actual_flavor:expr) => {
        $actual_flavor == $ask_flavor
    };
}

macro_rules! define_terminals {
    (
        identifiers { $($id_name:ident => $id_val:literal $(if $id_flavor:expr)?),+ }
        symbols { $($sy_name:ident => $sy_val:literal $(if $sy_flavor:expr)?),+ }
    ) => {
        /// An atomic symbol or reserved identifier.
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
        pub enum TerminalToken {
            $($id_name),+,
            $($sy_name),+
        }
        impl TerminalToken {
            /// All identifier terminals and their string representations.
            pub const IDENTIFIERS: &'static [(TerminalToken, &'static str)] = &[
                $((TerminalToken::$id_name, $id_val)),+
            ];
            /// All symbol terminals and their string representations.
            pub const SYMBOLS: &'static [(TerminalToken, &'static str)] = &[
                $((TerminalToken::$sy_name, $sy_val)),+
            ];

            /// `true` if this terminal is an identifier and not a symbol.
            pub fn is_identifier(self) -> bool {
                match self {
                    $(TerminalToken::$id_name => true),+,
                    $(TerminalToken::$sy_name => false),+
                }
            }

            /// `true` if this terminal is a symbol and not an identifier.
            pub fn is_symbol(self) -> bool {
                !self.is_identifier()
            }

            /// Returns the string representation of the terminal.
            pub fn as_str(self) -> &'static str {
                match self {
                    $(TerminalToken::$id_name => $id_val),+,
                    $(TerminalToken::$sy_name => $sy_val),+
                }
            }

            /// `true` if the terminal is supported by the Squirrel flavor.
            pub fn is_supported(self, flavor: Flavor) -> bool {
                match self {
                    $(TerminalToken::$id_name => _terminal_matches!(flavor $($id_flavor)?)),+,
                    $(TerminalToken::$sy_name => _terminal_matches!(flavor $($sy_flavor)?)),+
                }
            }
        }
    };
}

define_terminals! {
    identifiers {
        Break => "break",
        Case => "case",
        Catch => "catch",
        Class => "class",
        Clone => "clone",
        Continue => "continue",
        Const => "const",
        Default => "default",
        Delegate => "delegate",
        Delete => "delete",
        Do => "do",
        Else => "else",
        Enum => "enum",
        Extends => "extends",
        For => "for",
        Foreach => "foreach",
        Function => "function",
        If => "if",
        In => "in",
        Local => "local",
        Return => "return",
        Switch => "switch",
        Throw => "throw",
        Try => "try",
        Typeof => "typeof",
        While => "while",
        Yield => "yield",

        Constructor => "constructor",
        Instanceof => "instanceof",
        Static => "static",

        // _re extensions
        DelayThread => "delaythread"                        if Flavor::SquirrelRespawn,
        Expect => "expect"                                  if Flavor::SquirrelRespawn,
        FunctionRef => "functionref"                        if Flavor::SquirrelRespawn,
        Global => "global"                                  if Flavor::SquirrelRespawn,
        GlobalizeAllFunctions => "globalize_all_functions"  if Flavor::SquirrelRespawn,
        OrNull => "ornull"                                  if Flavor::SquirrelRespawn,
        Struct => "struct"                                  if Flavor::SquirrelRespawn,
        Thread => "thread"                                  if Flavor::SquirrelRespawn,
        Typedef => "typedef"                                if Flavor::SquirrelRespawn,
        Untyped => "untyped"                                if Flavor::SquirrelRespawn,
        WaitThread => "waitthread"                          if Flavor::SquirrelRespawn,
        WaitThreadSolo => "waitthreadsolo"                  if Flavor::SquirrelRespawn,
        Wait => "wait"                                      if Flavor::SquirrelRespawn
    }

    symbols {
        // Three-char symbols
        ThreeWay => "<=>",
        Ellipsis => "...",

        // Two-char symbols
        NotEqual => "!=",
        Equal => "==",
        LogicalOr => "||",
        LogicalAnd => "&&",
        GreaterEqual => ">=",
        LessEqual => "<=",
        AddEqual => "+=",
        SubtractEqual => "-=",
        DivideEqual => "/=",
        MultiplyEqual => "*=",
        ModuloEqual => "%=",
        Increment => "++",
        Decrement => "--",
        Namespace => "::",
        OpenAttributes => "</",
        CloseAttributes => "/>",

        // One-char symbols
        Not => "!",
        Greater => ">",
        Less => "<",
        Add => "+",
        Subtract => "-",
        Divide => "/",
        Multiply => "*",
        Modulo => "%",
        Assign => "=",
        BitwiseAnd => "&",
        BitwiseOr => "|",
        BitwiseXor => "^",
        BitwiseNot => "~",
        OpenBrace => "{",
        CloseBrace => "}",
        OpenSquare => "[",
        CloseSquare => "]",
        OpenBracket => "(",
        CloseBracket => ")",
        Dot => ".",
        Comma => ",",
        Colon => ":",
        Question => "?",
        Semicolon => ";",
        At => "@"
    }
}

/// A string literal.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StringToken<'s> {
    /// A literal string.
    ///
    /// # Example
    /// ```text
    /// "hello\nworld"
    /// ```
    Literal(&'s str),
    /// A verbatim string.
    ///
    /// # Example
    /// ```text
    /// @"hello
    /// world"
    /// ```
    Verbatim(&'s str),
    /// An asset string.
    ///
    /// # Example
    /// ```text
    /// $"hello world"
    /// ```
    Asset(&'s str),
}

/// The base of an [`Int`] literal.
///
/// [`Int`]: LiteralToken::Int
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralBase {
    Decimal,
    Octal,
    Hexadecimal,
}

/// A literal.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralToken<'s> {
    /// An integer value in some base.
    ///
    /// # Example
    /// ```text
    /// 7274    // decimal
    /// 0123    // octal
    /// 0xFF    // hexadecimal
    /// ```
    Int(i64, LiteralBase),

    /// A character literal.
    ///
    /// # Example
    /// ```text
    /// 'a'
    /// '\n'
    /// ```
    Char(&'s str),

    /// A floating-point literal.
    ///
    /// # Example
    /// ```text
    /// 1.0
    /// .53
    /// 23.
    /// 5e10
    /// ```
    Float(f64),

    /// A string literal.
    ///
    /// # Example
    /// ```text
    /// "a literal string"
    /// @"a verbatim string"
    /// $"an asset string"
    /// ```
    String(StringToken<'s>),
}

/// The type of a [`Token`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType<'s> {
    /// An empty value, for when there are no tokens to attach comments to.
    ///
    /// This will generally only occur at the end of input if there are remaining comments.
    Empty,

    /// A terminal token.
    ///
    /// # Example
    /// ```text
    /// function
    /// struct
    /// <=>
    /// ```
    Terminal(TerminalToken),

    /// A literal token.
    ///
    /// # Example
    /// ```text
    /// 7274
    /// 1.23
    /// 'a'
    /// "hello world"
    /// ```
    Literal(LiteralToken<'s>),

    /// An identifier.
    ///
    /// # Example
    /// ```text
    /// helloWorld
    /// this_is_my
    /// _list_of_identifiers
    /// ThereAre4
    /// ```
    Identifier(&'s str),
}

/// A comment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Comment<'s> {
    /// A multi-line comment. Doesn't necessarily actually span multiple lines.
    ///
    /// # Example
    /// ```text
    /// /* hello world */
    ///
    /// /* multiple
    /// lines */
    ///
    /// /**
    ///  * documentation
    ///  */
    /// ```
    MultiLine(&'s str),

    /// A C-style single line comment.
    ///
    /// # Example
    /// ```text
    /// // hello
    /// // world
    /// ```
    SingleLine(&'s str),

    /// A script-style single line comment, for e.g. preprocessor definitions.
    ///
    /// # Example
    /// ```text
    /// #ifdef CLIENT
    /// #endif
    /// ```
    ScriptStyle(&'s str),
}

/// A source-preserving token.
///
/// Each token contains a [`TokenType`], defining the actual parsed token, as well as a source
/// range and comment metadata.
///
/// # Comments
/// Consider this token with surrounding comments:
///
/// ```ignore
/// // this is my function
/// /* hi */ // there
/// /* here it comes: */ function // do you like it?
/// /* some other comment */
/// ```
///
/// Each comment in the source string ends up "owned" by a single token. Each token owns any
/// comments on empty lines before it, any comments on the same line before it, and any comments
/// on the same line after it if the token is at the end of the line. In general this is meant to
/// match how comments are normally attached to pieces of code.
///
/// In the example above, the `function` token would end up looking like this:
/// ```
/// use sqparse::token::{Comment, TerminalToken, Token, TokenLine, TokenType};
/// let _ = Token {
///     ty: TokenType::Terminal(TerminalToken::Function),
///     range: 65..73,
///
///     before_lines: vec![
///         TokenLine { comments: vec![Comment::SingleLine("this is my function")] },
///         TokenLine { comments: vec![Comment::MultiLine("hi"), Comment::SingleLine("there")] },
///     ],
///     comments: vec![
///         Comment::MultiLine("here it comes:")
///     ],
///     new_line: Some(TokenLine { comments: vec![Comment::SingleLine("do you like it?")] })
/// };
/// ```
///
/// Notice how the token keeps track of the separate lines of comments before it, and whether it is
/// followed by a newline. Also notice that the last `/* some other comment */` isn't included,
/// since it is on a different line after the token it would be owned by whatever token comes later.
///
/// If there are un-owned comments at the end of an input when parsing completes, an [`Empty`] token
/// will be emitted.
///
/// [`Empty`]: TokenType::Empty
#[derive(Clone, Debug, PartialEq)]
#[must_use]
pub struct Token<'s> {
    /// The type of token.
    pub ty: TokenType<'s>,

    /// The character range of the token in the source string.
    pub range: Range<usize>,

    /// Empty lines that appear before the token. The lines may contain comments.
    pub before_lines: Vec<TokenLine<'s>>,

    /// Comments that appear before the token on the same line.
    pub comments: Vec<Comment<'s>>,

    /// If this token ends a line, includes any comments between the token and newline.
    pub new_line: Option<TokenLine<'s>>,
}

/// A line of source input in a [`Token`].
#[derive(Clone, Debug, PartialEq)]
pub struct TokenLine<'s> {
    /// Comments that appear before the newline.
    pub comments: Vec<Comment<'s>>,
}

impl std::fmt::Display for LiteralToken<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralToken::Int(_, _) => write!(f, "an integer literal"),
            LiteralToken::Char(_) => write!(f, "a character literal"),
            LiteralToken::Float(_) => write!(f, "a float literal"),
            LiteralToken::String(_) => write!(f, "a string literal"),
        }
    }
}

impl std::fmt::Display for TokenType<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Empty => write!(f, "<empty>"),
            TokenType::Terminal(terminal) => write!(f, "`{}`", terminal.as_str()),
            TokenType::Literal(literal) => write!(f, "{literal}"),
            TokenType::Identifier(text) => write!(f, "`{text}`"),
        }
    }
}
