# sqparse

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build status][build-badge]][build-url]

[crates-badge]: https://img.shields.io/crates/v/sqparse.svg
[crates-url]: https://crates.io/crates/sqparse
[docs-badge]: https://img.shields.io/docsrs/sqparse
[docs-url]: https://docs.rs/sqparse/
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/cpdt/sqparse/blob/master/LICENSE
[build-badge]: https://github.com/cpdt/sqparse/workflows/Build/badge.svg
[build-url]: https://github.com/cpdt/sqparse/actions?query=workflow%3ABuild+branch%3Amain

This is a parser for the [Squirrel language](http://squirrel-lang.org/), written in Rust. It is primarily designed to
parse [Respawn's custom Squirrel dialect](https://noskill.gitbook.io/titanfall2/documentation/file-format/nut-and-gnut-squirrel),
but should be able to handle Squirrel 2 and 3 code as well.

Features:

 - Completely source-preserving: all tokens and comments in the input string are included in the AST. This makes it
   perfect for source modification operations like code formatting.
 - Friendly error messages: in general, the parser aims to show nice syntax error messages with useful contextual
   information. Unfortunately this isn't always possible due to syntax ambiguities, especially where Respawn's type
   system is involved.
 - Parses all [Northstar scripts](https://github.com/R2Northstar/NorthstarMods) and
   [R5Reloaded scripts](https://github.com/Mauler125/scripts_r5) successfully. The resulting ASTs have not been
   verified.

There are probably bugs.

## Examples

There are some examples included. Use cargo to run them:

 - Print AST debug output:
   ```
   $ cargo run --example print_ast
   ```
 - Print an example lexer error:
   ```
   $ cargo run --example print_lexer_error
   ```
 - Print an example parser error:
   ```
   $ cargo run --example print_parser_error
   ```
 - Dry-run the parser on one file or a directory tree:
   ```
   $ cargo run --release --example dryrun -- [path to file or directory]
   ```
