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
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
        pub enum TerminalToken {
            $($id_name),+,
            $($sy_name),+
        }
        impl TerminalToken {
            pub const IDENTIFIERS: &'static [(TerminalToken, &'static str)] = &[
                $((TerminalToken::$id_name, $id_val)),+
            ];
            pub const SYMBOLS: &'static [(TerminalToken, &'static str)] = &[
                $((TerminalToken::$sy_name, $sy_val)),+
            ];

            pub fn is_identifier(self) -> bool {
                match self {
                    $(TerminalToken::$id_name => true),+,
                    $(TerminalToken::$sy_name => false),+
                }
            }

            pub fn as_str(self) -> &'static str {
                match self {
                    $(TerminalToken::$id_name => $id_val),+,
                    $(TerminalToken::$sy_name => $sy_val),+
                }
            }

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
        Var => "var"                                        if Flavor::SquirrelRespawn,
        WaitThread => "waitthread"                          if Flavor::SquirrelRespawn,
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
        Semicolon => ";"
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StringToken<'s> {
    Literal(&'s str),
    Verbatim(&'s str),
    Asset(&'s str),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralBase {
    Decimal,
    Octal,
    Hexadecimal,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralToken<'s> {
    Int(i64, LiteralBase),
    Char(&'s str),
    Float(f64),
    String(StringToken<'s>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType<'s> {
    Empty,
    Terminal(TerminalToken),
    Literal(LiteralToken<'s>),
    Identifier(&'s str),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Comment<'s> {
    MultiLine(&'s str),
    SingleLine(&'s str),
    ScriptStyle(&'s str),
}

#[derive(Clone, Debug, PartialEq)]
#[must_use]
pub struct Token<'s> {
    pub ty: TokenType<'s>,
    pub range: Range<usize>,

    // comments that appear before the token
    pub comments: Vec<Comment<'s>>,

    // empty lines (that contain only comments) that appear before the token
    pub before_lines: Vec<TokenLine<'s>>,

    // is there a new line after this token?
    pub new_line: Option<TokenLine<'s>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenLine<'s> {
    // comments that appear before the newline
    pub comments: Vec<Comment<'s>>,
}

impl<'s> Token<'s> {
    pub fn is_terminal(&self, terminal: TerminalToken) -> bool {
        match &self.ty {
            TokenType::Terminal(id) => *id == terminal,
            _ => false,
        }
    }
}

impl std::fmt::Display for LiteralToken<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralToken::Int(_, _) => write!(f, "integer literal"),
            LiteralToken::Char(_) => write!(f, "character literal"),
            LiteralToken::Float(_) => write!(f, "float literal"),
            LiteralToken::String(_) => write!(f, "string literal"),
        }
    }
}

impl std::fmt::Display for TokenType<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Empty => write!(f, "<empty>"),
            TokenType::Terminal(terminal) => write!(f, "`{}`", terminal.as_str()),
            TokenType::Literal(literal) => write!(f, "{literal}"),
            TokenType::Identifier(_) => write!(f, "identifier"),
        }
    }
}
