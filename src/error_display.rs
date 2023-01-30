use std::fmt::Formatter;
use std::ops::Range;

struct ErrorDisplay<'s> {
    range: Range<usize>,
    source: &'s str,
}

pub fn display_error(range: Range<usize>, source: &str) -> impl std::fmt::Display + '_ {
    ErrorDisplay { range, source }
}

impl<'s> std::fmt::Display for ErrorDisplay<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let line_start = self.source[..self.range.start]
            .rfind('\n')
            .map(|idx| idx + 1)
            .unwrap_or(0);
        let line_end = self.source[self.range.end..]
            .find('\n')
            .map(|idx| self.range.end + idx)
            .unwrap_or(self.source.len());

        let lines_text = &self.source[line_start..line_end];

        let first_line_number = count_newlines(&self.source[..line_start]) + 1;
        let last_line_number = first_line_number + count_newlines(lines_text);

        let longest_line_number_len = last_line_number.to_string().len();

        // If the error spans a single line, just print that line.
        if first_line_number == last_line_number {
            let line_no_tabs = lines_text.replace('\t', " ");
            writeln!(
                f,
                " {line_no_pad} |",
                line_no_pad = repeat(longest_line_number_len, ' '),
            )?;
            writeln!(f, " {first_line_number} | {line_no_tabs}",)?;
            write!(
                f,
                " {line_no_pad} | {offset_pad}{underline}",
                line_no_pad = repeat(longest_line_number_len, ' '),
                offset_pad = repeat(self.range.start - line_start, ' '),
                underline = repeat(self.range.len().max(1), '^'),
            )?;
            return Ok(());
        }

        // If the error spans multiple lines, enable fancy printing.
        writeln!(
            f,
            " {line_no_pad} |",
            line_no_pad = repeat(longest_line_number_len, ' '),
        )?;
        for (line_offset, line) in lines_text.split('\n').enumerate() {
            let prefix = if line_offset == 0 { ' ' } else { '|' };
            let line_no_tabs = line.replace('\t', " ");

            writeln!(
                f,
                " {line_no: >line_no_len$} | {prefix} {line}",
                line_no = first_line_number + line_offset,
                line_no_len = longest_line_number_len,
                prefix = prefix,
                line = line_no_tabs,
            )?;

            if line_offset == 0 {
                writeln!(
                    f,
                    " {line_no_pad} |  {offset_pad}^",
                    line_no_pad = repeat(longest_line_number_len, ' '),
                    offset_pad = repeat(self.range.start - line_start + 1, '_')
                )?;
            }
        }

        let last_line_start = self.source[..self.range.end]
            .rfind('\n')
            .map(|idx| idx + 1)
            .unwrap_or(0);
        write!(
            f,
            " {line_no_pad} | |{offset_pad}^",
            line_no_pad = repeat(longest_line_number_len, ' '),
            offset_pad = repeat(self.range.end - last_line_start, '_')
        )?;

        Ok(())
    }
}

fn count_newlines(text: &str) -> usize {
    text.chars().filter(|c| *c == '\n').count()
}

// Small util for repeating a character
#[derive(Clone, Copy)]
struct DisplayRepeat<T>(usize, T);

impl<T: std::fmt::Display> std::fmt::Display for DisplayRepeat<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.0 {
            self.1.fmt(f)?;
        }
        Ok(())
    }
}

fn repeat<T>(times: usize, item: T) -> DisplayRepeat<T> {
    DisplayRepeat(times, item)
}
