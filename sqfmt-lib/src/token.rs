use sqparse::Token;
use sqparse::token::{Comment, LiteralBase, LiteralToken, StringToken, TerminalToken, TokenLine, TokenType};
use crate::combinators::{alt, new_line, empty_line, pair, single_line, space, tuple, iter, cond_or};
use crate::comment::comment;
use crate::writer::Writer;

pub fn token<'s>(token: &'s Token<'s>) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    move |mut i| {
        i = token_before_lines(&token.before_lines)(i)?;

        i = cond_or(
            token.comments.is_empty(),
            token_type(token.ty),
            alt(
                single_line(pair(single_line_comment_list(&token.comments), token_type(token.ty))),
                tuple((
                    multi_line_comment_list(&token.comments),
                    empty_line,
                    token_type(token.ty),
                )),
            )
        )(i)?;

        if let Some(line) = &token.new_line {
            i = pair(space, inline_comment_list(&line.comments))(i)?;
        }

        Some(i)
    }
}

// Prints comments around a token, but ignores the token itself
pub fn discard_token<'s>(token: &'s Token<'s>) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    move |mut i| {
        if token.new_line.is_some() && i.is_single_line() {
            return None;
        }

        i = pair(
            token_before_lines(&token.before_lines),
            inline_comment_list(&token.comments),
        )(i)?;

        if let Some(line) = &token.new_line {
            i = tuple((space, inline_comment_list(&line.comments), empty_line))(i)?;
        }

        Some(i)
    }
}

fn token_before_lines<'s>(lines: &'s [TokenLine<'s>]) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    move |mut i| {
        let before_lines_iter = lines.iter().skip_while(|line| line.comments.is_empty());

        let mut was_last_empty = false;
        for before_line in before_lines_iter {
            // Skip consecutive empty lines
            if was_last_empty && before_line.comments.is_empty() {
                continue;
            }
            was_last_empty = before_line.comments.is_empty();
            if was_last_empty {
                i = new_line(i)?;
            } else {
                i = pair(inline_comment_list(&before_line.comments), empty_line)(i)?;
            }
        }

        Some(i)
    }
}

fn single_line_comment_list<'s>(comments: &'s [Comment<'s>]) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    iter(comments.iter().map(|c| tuple((space, comment(c), space))))
}

fn multi_line_comment_list<'s>(comments: &'s [Comment<'s>]) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    iter(comments.iter().map(|c| tuple((comment(c), empty_line))))
}

fn inline_comment_list<'s>(comments: &'s [Comment<'s>]) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    alt(
        single_line(single_line_comment_list(comments)),
        multi_line_comment_list(comments),
    )
}

fn token_type<'s>(token_ty: TokenType<'s>) -> impl FnOnce(Writer) -> Option<Writer> +'s {
    move |i| {
        match token_ty {
            TokenType::Empty => Some(i),
            TokenType::Terminal(terminal) => terminal_token(terminal)(i),
            TokenType::Literal(literal) => literal_token(literal)(i),
            TokenType::Identifier(identifier) => i.write(identifier),
        }
    }
}

fn terminal_token(terminal: TerminalToken) -> impl FnOnce(Writer) -> Option<Writer> {
    move |i| i.write(terminal.as_str())
}

fn int_to_base_string(val: i64, base: LiteralBase) -> String {
    match base {
        LiteralBase::Decimal => val.to_string(),
        LiteralBase::Octal => format!("0{:o}", val),
        LiteralBase::Hexadecimal => format!("{:#x}", val),
    }
}

fn literal_token<'s>(literal: LiteralToken<'s>) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    move |i| {
        match literal {
            LiteralToken::Int(val, base) => i.write(&int_to_base_string(val, base)),
            LiteralToken::Char(val) => i.write(&format!("'{}'", val)),
            LiteralToken::Float(val) => i.write(&lexical::to_string(val)),
            LiteralToken::String(StringToken::Literal(val)) => i.write(&format!("\"{}\"", val)),
            LiteralToken::String(StringToken::Verbatim(val)) => i.write(&format!("@\"{}\"", val)),
            LiteralToken::String(StringToken::Asset(val)) => i.write(&format!("$\"{}\"", val)),
        }
    }
}

#[cfg(test)]
mod test {
    use sqparse::Token;
    use sqparse::token::{Comment, LiteralBase, LiteralToken, StringToken, TerminalToken, TokenLine, TokenType};
    use crate::test_utils::{mock_token, test_write, test_write_columns};
    use crate::token::{token, token_type};

    #[test]
    fn empty_token() {
        let t = TokenType::Empty;
        let val = test_write(token_type(t));

        assert_eq!(val, "");
    }

    #[test]
    fn terminal_identifier_token() {
        let t = TokenType::Terminal(TerminalToken::Throw);
        let val = test_write(token_type(t));

        assert_eq!(val, "throw");
    }

    #[test]
    fn terminal_symbol_token() {
        let t = TokenType::Terminal(TerminalToken::ThreeWay);
        let val = test_write(token_type(t));

        assert_eq!(val, "<=>");
    }

    #[test]
    fn literal_int_token() {
        let t = TokenType::Literal(LiteralToken::Int(123, LiteralBase::Decimal));
        let val = test_write(token_type(t));
        assert_eq!(val, "123");

        let t = TokenType::Literal(LiteralToken::Int(123, LiteralBase::Hexadecimal));
        let val = test_write(token_type(t));
        assert_eq!(val, "0x7b");

        let t = TokenType::Literal(LiteralToken::Int(123, LiteralBase::Octal));
        let val = test_write(token_type(t));
        assert_eq!(val, "0173");
    }

    #[test]
    fn literal_char_token() {
        let t = TokenType::Literal(LiteralToken::Char("ABC"));
        let val = test_write(token_type(t));
        assert_eq!(val, "'ABC'");
    }

    #[test]
    fn literal_float_token() {
        let t = TokenType::Literal(LiteralToken::Float(123.45678e9));
        let val = test_write(token_type(t));
        assert_eq!(val, "1.2345678e11");
    }

    #[test]
    fn literal_string_token() {
        let t = TokenType::Literal(LiteralToken::String(StringToken::Literal("hello world!")));
        let val = test_write(token_type(t));
        assert_eq!(val, "\"hello world!\"");

        let t = TokenType::Literal(LiteralToken::String(StringToken::Verbatim("hello world!")));
        let val = test_write(token_type(t));
        assert_eq!(val, "@\"hello world!\"");

        let t = TokenType::Literal(LiteralToken::String(StringToken::Asset("hello world!")));
        let val = test_write(token_type(t));
        assert_eq!(val, "$\"hello world!\"");
    }

    #[test]
    fn identifier_token() {
        let t = TokenType::Identifier("my_cool_thing98");
        let val = test_write(token_type(t));
        assert_eq!(val, "my_cool_thing98");
    }

    #[test]
    fn token_with_no_comments() {
        let t = Token {
            ty: TokenType::Identifier("hello"),
            range: 0..0,
            comments: Vec::new(),
            before_lines: Vec::new(),
            new_line: None,
        };
        let val = test_write(token(&t));
        assert_eq!(val, "hello");
    }

    #[test]
    fn token_with_new_line() {
        let t = Token {
            ty: TokenType::Identifier("hello"),
            range: 0..0,
            comments: Vec::new(),
            before_lines: Vec::new(),
            new_line: Some(TokenLine { comments: Vec::new() }),
        };
        let val = test_write(token(&t));
        assert_eq!(val, "hello");
    }

    #[test]
    fn token_with_inline_comment_before() {
        let t = Token {
            ty: TokenType::Identifier("hello"),
            range: 0..0,
            comments: vec![
                Comment::MultiLine("Hello world!")
            ],
            before_lines: Vec::new(),
            new_line: None,
        };
        let val = test_write_columns(80, token(&t));
        assert_eq!(val, "/* Hello world! */ hello");
    }

    #[test]
    fn token_with_multiple_inline_comments_before() {
        let t = Token {
            ty: TokenType::Identifier("hello"),
            range: 0..0,
            comments: vec![
                Comment::MultiLine("Hello!"),
                Comment::MultiLine("world!"),
                Comment::MultiLine("woah there")
            ],
            before_lines: Vec::new(),
            new_line: None,
        };
        let val = test_write_columns(100, token(&t));
        assert_eq!(val, "/* Hello! */ /* world! */ /* woah there */ hello");
    }

    #[test]
    fn token_with_wrapping_inline_comments_before() {
        let t = Token {
            ty: TokenType::Identifier("hello"),
            range: 0..0,
            comments: vec![
                Comment::MultiLine("Short comment"),
                Comment::MultiLine("*\nThis is a doc comment. It also has a lot of text, so it will span multiple lines."),
                Comment::MultiLine("Also short")
            ],
            before_lines: Vec::new(),
            new_line: None,
        };
        let val = test_write(token(&t));
        assert_eq!(val, r#"
/* Short comment */
/**
 * This is a doc
 * comment. It also
 * has a lot of
 * text, so it will
 * span multiple
 * lines.
 */
/* Also short */
hello"#.trim_start());
    }

    #[test]
    fn token_with_line_comment_before() {
        let t = Token {
            ty: TokenType::Identifier("hello"),
            range: 0..0,
            comments: vec![
                Comment::SingleLine("Hello world!")
            ],
            before_lines: Vec::new(),
            new_line: None,
        };
        let val = test_write(token(&t));
        assert_eq!(val, "// Hello world!\nhello");
    }

    #[test]
    fn token_with_mixed_comments_before() {
        let t = Token {
            ty: TokenType::Identifier("hello"),
            range: 0..0,
            comments: vec![
                Comment::MultiLine("Short comment"),
                Comment::SingleLine("Woah there"),
                Comment::ScriptStyle("Scripts!"),
                Comment::MultiLine("more multi lines")
            ],
            before_lines: Vec::new(),
            new_line: None,
        };
        let val = test_write(token(&t));
        assert_eq!(val, r#"
/* Short comment */
// Woah there
# Scripts!
/**
 * more multi lines
 */
hello"#.trim_start());
    }

    #[test]
    fn token_with_before_lines() {
        let t = Token {
            ty: TokenType::Identifier("hello"),
            range: 0..0,
            comments: Vec::new(),
            before_lines: vec![
                TokenLine { comments: Vec::new() },
                TokenLine { comments: Vec::new() },
                TokenLine { comments: vec![Comment::SingleLine("nice!")] },
                TokenLine { comments: Vec::new() },
                TokenLine { comments: Vec::new() },
                TokenLine { comments: Vec::new() },
                TokenLine { comments: vec![Comment::MultiLine("one comment"), Comment::SingleLine("another comment")] },
                TokenLine { comments: vec![Comment::SingleLine("and a third comment")] },
            ],
            new_line: None,
        };
        let val = test_write(token(&t));
        assert_eq!(val, r#"
// nice!

/* one comment */
// another comment
// and a third
// comment
hello"#.trim_start());
    }

    #[test]
    fn token_with_before_lines_and_comment() {
        let t = Token {
            ty: TokenType::Identifier("hello"),
            range: 0..0,
            comments: vec![Comment::MultiLine("yo!")],
            before_lines: vec![
                TokenLine { comments: vec![Comment::MultiLine("nice!")] },
            ],
            new_line: None,
        };
        let val = test_write(token(&t));
        assert_eq!(val, "/* nice! */\n/* yo! */ hello");
    }

    #[test]
    fn token_with_new_line_comment() {
        let t = Token {
            ty: TokenType::Identifier("hello"),
            range: 0..0,
            comments: Vec::new(),
            before_lines: Vec::new(),
            new_line: Some(TokenLine { comments: vec![Comment::MultiLine("yes!")] }),
        };
        let val = test_write(token(&t));
        assert_eq!(val, "hello /* yes! */");
    }

    #[test]
    fn token_with_new_line_single_comment() {
        let t = Token {
            ty: TokenType::Identifier("hello"),
            range: 0..0,
            comments: Vec::new(),
            before_lines: Vec::new(),
            new_line: Some(TokenLine { comments: vec![Comment::SingleLine("yes!")] }),
        };
        let val = test_write(token(&t));
        assert_eq!(val, "hello // yes!\n");
    }

    #[test]
    fn token_with_new_line_wrapping_comment() {
        let t = Token {
            ty: TokenType::Identifier("hello"),
            range: 0..0,
            comments: Vec::new(),
            before_lines: Vec::new(),
            new_line: Some(TokenLine { comments: vec![Comment::SingleLine("yes, this is some longer form text.")] }),
        };
        let val = test_write(token(&t));
        assert_eq!(val, r#"
hello // yes, this
// is some longer
// form text.
"#.trim_start());
    }

    #[test]
    fn token_with_new_line_multiple_comments() {
        let t = Token {
            ty: TokenType::Identifier("hello"),
            range: 0..0,
            comments: Vec::new(),
            before_lines: Vec::new(),
            new_line: Some(TokenLine { comments: vec![Comment::MultiLine("first"), Comment::MultiLine("second"), Comment::SingleLine("third"), Comment::MultiLine("fourth"), Comment::ScriptStyle("fifth")] }),
        };
        let val = test_write(token(&t));
        assert_eq!(val, r#"
hello /* first */
/* second */
// third
/* fourth */
# fifth
"#.trim_start());
    }
}
