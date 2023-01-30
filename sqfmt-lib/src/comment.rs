use sqparse::token::Comment;
use crate::combinators::{definitely_multi_line, Formatter, empty_line, pair};
use crate::writer::Writer;

const SINGLE_LINE_START: &'static str = "// ";
const SCRIPT_LINE_START: &'static str = "# ";
const SINGLE_MULTI_START: &'static str = "/* ";
const SINGLE_MULTI_END: &'static str = " */";
const DOC_OPEN: &'static str = "/**";
const DOC_LINE_START: &'static str = " * ";
const DOC_CLOSE: &'static str = " */";

pub fn comment<'s>(comment: &'s Comment<'s>) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    move |i| {
        match comment {
            Comment::MultiLine(val) => {
                match strip_doc_prefix(*val) {
                    Some(doc_text) => doc_comment(TextWrapIter::new(doc_text, true))(i),
                    None => multi_line_comment(*val)(i),
                }
            },
            Comment::SingleLine(val) => single_line_comment(SINGLE_LINE_START, *val)(i),
            Comment::ScriptStyle(val) => single_line_comment(SCRIPT_LINE_START, *val)(i),
        }
    }
}

fn multi_line_comment<'s>(val: &'s str) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    move |i| {
        // Attempt to lay out on one line
        let mut iter = TextWrapIter::new(val, false);
        let columns = i.remaining_columns().saturating_sub(SINGLE_MULTI_START.len()).saturating_sub(SINGLE_MULTI_END.len());
        let single_line = match iter.next(columns) {
            Some(line) => line,
            None => return Some(i),
        };

        if iter.is_done() {
            i
                .write(SINGLE_MULTI_START)?
                .write(single_line)?
                .write(SINGLE_MULTI_END)
        } else {
            doc_comment(TextWrapIter::new(val, false))(i)
        }
    }
}

fn doc_comment<'s>(mut line_iter: TextWrapIter<'s>) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    definitely_multi_line(pair(
        empty_line,
        move |mut i: Writer| {
            i = i
                .write(DOC_OPEN)?
                .empty_line()?;

            while let Some(line_text) = line_iter.next(i.remaining_columns().saturating_sub(DOC_LINE_START.len())) {
                i = i
                    .write(DOC_LINE_START)?
                    .write(line_text)?
                    .empty_line()?;
            }

            i
                .write(DOC_CLOSE)?
                .empty_line()
        }
    ))
}

fn single_line_comment<'s>(line_start: &'s str, val: &'s str) -> impl FnOnce(Writer) -> Option<Writer> + 's {
    definitely_multi_line(move |mut i| {
        let mut line_iter = TextWrapIter::new(val, false);
        while let Some(line_text) = line_iter.next(i.remaining_columns().saturating_sub(line_start.len())) {
            i = i
                .write(line_start)?
                .write(line_text)?
                .write_new_line()?;
        }

        Some(i)
    })
}

// todo: this needs to handle tabs in the text correctly
#[derive(Clone, Copy)]
pub struct TextWrapIter<'s> {
    text: Option<&'s str>,
    is_doc_comment: bool,
}

impl<'s> TextWrapIter<'s> {
    fn new(text: &'s str, is_doc_comment: bool) -> Self {
        let text = if is_doc_comment {
            remove_doc_line_start(text)
        } else {
            trim_line_start(text)
        };
        let text = text.trim_end();

        TextWrapIter { text: Some(text), is_doc_comment }
    }

    fn next(&mut self, columns: usize) -> Option<&'s str> {
        let text = self.text?;

        let max_end = (columns + 1).min(text.len());
        let next_line_max = &text[..max_end];
        let (is_new_line, break_pos) = match next_line_max.find('\n') {
            Some(pos) => (true, pos),
            None if text.len() <= columns => (false, text.len()),
            None => (
                false,

                // Break at the first word boundary before the end of the line.
                // Or if there isn't one, the first word boundary after the end of the line.
                next_line_max
                    .rfind(char::is_whitespace)
                    .unwrap_or_else(|| {
                        text[max_end..]
                            .find(char::is_whitespace)
                            .map(|offset| max_end + offset)
                            .unwrap_or(text.len())
                    })
            ),
        };

        let (this_line, remaining) = text.split_at(break_pos);
        let new_text = if is_new_line && self.is_doc_comment {
            remove_doc_line_start(remaining)
        } else {
            trim_line_start(remaining)
        };
        self.text = if new_text.is_empty() { None } else { Some(new_text) };
        Some(this_line.trim_end())
    }

    fn is_done(&self) -> bool {
        self.text.is_none()
    }
}

fn trim_line_start(line: &str) -> &str {
    line
        .strip_prefix('\n')
        .unwrap_or(line)
        .trim_start_matches(|c: char| c != '\n' && c.is_whitespace())
}

fn remove_doc_line_start(line: &str) -> &str {
    let line = trim_line_start(line);
    let line = line.strip_prefix('*').unwrap_or(line);
    line.strip_prefix(' ').unwrap_or(line)
}

fn strip_doc_prefix(doc_val: &str) -> Option<&str> {
    let doc_val = doc_val.strip_prefix('*')?;
    let newline_index = doc_val.find(|c: char| c == '\n' || !c.is_whitespace())?;
    doc_val[newline_index..].strip_prefix('\n')
}

#[cfg(test)]
mod test {
    use sqparse::token::Comment;
    use crate::comment::comment;
    use crate::test_utils::{test_write, test_write_columns};

    #[test]
    fn single_line_no_wrapping() {
        let c = Comment::SingleLine("    Hello world!  ");
        let val = test_write(comment(&c));

        assert_eq!(val, "// Hello world!\n");
    }

    #[test]
    fn single_line_wrapping() {
        let c = Comment::SingleLine("0 1 2 3 4 5 6 7 8 9 This comment is over 20 columns wide");
        let val = test_write(comment(&c));

        assert_eq!(val, "// 0 1 2 3 4 5 6 7 8\n// 9 This comment is\n// over 20 columns\n// wide\n");
    }

    #[test]
    fn single_line_long_words() {
        let c = Comment::SingleLine("Thiswordisover10chars Thiswordisalsoover10chars ok? Andsoisthisone");
        let val = test_write(comment(&c));

        assert_eq!(val, "// Thiswordisover10chars\n// Thiswordisalsoover10chars\n// ok?\n// Andsoisthisone\n");
    }

    #[test]
    fn single_line_no_column() {
        let c = Comment::SingleLine("Hello world, this is some text");
        let val = test_write_columns(0, comment(&c));

        assert_eq!(val, "// Hello\n// world,\n// this\n// is\n// some\n// text\n");
    }

    #[test]
    fn script_no_wrapping() {
        let c = Comment::ScriptStyle("    Hello world!  ");
        let val = test_write(comment(&c));

        assert_eq!(val, "# Hello world!\n");
    }

    #[test]
    fn script_wrapping() {
        let c = Comment::ScriptStyle("0 1 2 3 4 5 6 78 9 This comment is over 20 columns wide");
        let val = test_write(comment(&c));

        assert_eq!(val, "# 0 1 2 3 4 5 6 78 9\n# This comment is\n# over 20 columns\n# wide\n");
    }

    #[test]
    fn multiline_single_world() {
        let c = Comment::MultiLine("Hello");
        let val = test_write(comment(&c));

        assert_eq!(val, "/* Hello */");
    }

    #[test]
    fn multiline_single_line() {
        let c = Comment::MultiLine("Hello world!");
        let val = test_write(comment(&c));

        assert_eq!(val, "/* Hello world! */");
    }

    #[test]
    fn multiline_force_breaks() {
        let c = Comment::MultiLine(" Hello  \n      world! ");
        let val = test_write(comment(&c));

        assert_eq!(val, "/**\n * Hello\n * world!\n */\n");
    }

    #[test]
    fn multiline_wrapping() {
        let c = Comment::MultiLine("0 1 2 3 4 5 6 7 8 9 This comment is over 20 columns wide");
        let val = test_write(comment(&c));

        assert_eq!(val, "/**\n * 0 1 2 3 4 5 6 7 8\n * 9 This comment is\n * over 20 columns\n * wide\n */\n");
    }

    #[test]
    fn multiline_doesnt_remove_doc_prefixes() {
        let c = Comment::MultiLine(" * Hello\n * world! ");
        let val = test_write(comment(&c));

        assert_eq!(val, "/**\n * * Hello\n * * world!\n */\n");
    }

    #[test]
    fn doc_removes_prefixes() {
        let c = Comment::MultiLine("*\n * Hello\n * world! ");
        let val = test_write(comment(&c));

        assert_eq!(val, "/**\n * Hello\n * world!\n */\n");
    }
}
