use std::fmt::Formatter;
use std::ops::Range;

pub struct ErrorDisplay<'s> {
    range: Range<usize>,
    source: &'s str,
}

pub fn error(range: Range<usize>, source: &str) -> ErrorDisplay {
    ErrorDisplay { range, source }
}

impl<'s> std::fmt::Display for ErrorDisplay<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let line_start = self.source[..self.range.start]
            .rfind('\n')
            .map(|idx| idx + 1)
            .unwrap_or(0);
        let line_end = self.source[self.range.start..]
            .find('\n')
            .map(|idx| self.range.start + idx)
            .unwrap_or(self.source.len());

        let line_number = self.source[..line_start]
            .chars()
            .filter(|c| *c == '\n')
            .count()
            + 1;
        let line_number_str = line_number.to_string();
        let line = self.source[line_start..line_end].replace('\t', " ");

        writeln!(
            f,
            " {line_no} | {line}",
            line_no = line_number_str,
            line = line,
        )?;
        write!(
            f,
            " {line_no_pad} | {offset_pad}{underline}",
            line_no_pad = repeat(line_number_str.len(), ' '),
            offset_pad = repeat(self.range.start - line_start, ' '),
            underline = repeat(self.range.len(), '^'),
        )
    }
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
