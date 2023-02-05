//! This is a parser for the [Squirrel language](http://squirrel-lang.org/), written in Rust. It is
//! primarily designed to parse [Respawn's custom Squirrel dialect](https://noskill.gitbook.io/titanfall2/documentation/file-format/nut-and-gnut-squirrel),
//! but should be able to handle Squirrel 2 and 3 code as well.
//!
//! Features:
//!  - Completely source-preserving: all tokens and comments in the input string are included in the
//!    AST. This makes it perfect for source modification operations like code formatting.
//!  - Friendly error messages: in general, the parser aims to show nice syntax error messages with
//!    useful contextual information. Unfortunately this isn't always possible due to syntax
//!    ambiguities, especially where Respawn's type system is involved.
//!  - Parses all [Northstar scripts](https://github.com/R2Northstar/NorthstarMods) and
//!    [R5Reloaded scripts](https://github.com/Mauler125/scripts_r5) successfully. The resulting
//!    ASTs have not been verified.
//!
//! # Example
//! ```
//! use sqparse::{Flavor, parse, tokenize};
//!
//! let source = r#"
//! global function MyFunction
//!
//! struct {
//!     int a
//! } file
//!
//! string function MyFunction( List<number> values ) {
//!     values.push(1 + 2)
//! }
//! "#;
//! let tokens = tokenize(source, Flavor::SquirrelRespawn).unwrap();
//!
//! let program = parse(&tokens).unwrap();
//! println!("Program: {:#?}", program);
//! ```

#![deny(missing_debug_implementations)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]

mod annotation;
pub mod ast;
mod flavor;
mod lexer;
mod parser;
pub mod token;

pub use self::annotation::{display_annotations, Annotation};
pub use self::flavor::Flavor;
pub use self::lexer::{tokenize, LexerError, LexerErrorType, TokenItem};
pub use self::parser::{parse, ContextType, ParseError, ParseErrorContext, ParseErrorType};
