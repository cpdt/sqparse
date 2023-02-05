use crate::error_display::gutter::Gutter;
use crate::error_display::line_printer::LinePrinter;
use std::fmt::{Display, Formatter};
use std::ops::{Range, RangeInclusive};

pub fn single_line_format(text: &str, highlight: Range<usize>) -> impl Display + '_ {
    let (line_number, line_start_index) = get_line_containing(highlight.start, text);
    let text_from_start = &text[line_start_index..];

    let end_index = text_from_start.find('\n').unwrap_or(text_from_start.len());
    let line = &text_from_start[..end_index];

    let line_highlight = (highlight.start - line_start_index)..(highlight.end - line_start_index);
    SingleLineFormatDisplay {
        line,
        line_number,
        line_highlight,
    }
}

pub fn multi_line_format(
    text: &str,
    highlight: Range<usize>,
    must_be_visible_range: Range<usize>,
) -> impl Display + '_ {
    let (first_line_number, first_start_index) = get_line_containing(highlight.start, text);
    let (last_line_number, last_start_index) = get_line_containing(highlight.end - 1, text);

    let (first_must_be_visible_line_number, _) =
        get_line_containing(must_be_visible_range.start, text);
    let (last_must_be_visible_line_number, _) =
        get_line_containing(must_be_visible_range.end - 1, text);

    let last_end_index = text[last_start_index..]
        .find('\n')
        .map(|idx| last_start_index + idx)
        .unwrap_or(text.len());

    let lines = &text[first_start_index..last_end_index];
    let first_line_highlight = highlight.start - first_start_index;
    let last_line_highlight = highlight.end - 1 - last_start_index;

    MultiLineFormatDisplay {
        line_numbers: first_line_number..=last_line_number,
        must_be_visible_line_numbers: first_must_be_visible_line_number
            ..=last_must_be_visible_line_number,
        lines,
        first_line_highlight,
        last_line_highlight,
    }
}

struct SingleLineFormatDisplay<'s> {
    line: &'s str,
    line_number: usize,
    line_highlight: Range<usize>,
}

impl Display for SingleLineFormatDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let gutter = Gutter::new(self.line_number);
        let mut printer = LinePrinter::new(gutter, self.line_number, 0);

        writeln!(f, "{}", gutter.empty())?;
        writeln!(f, "{}", printer.line(self.line))?;
        write!(f, "{}", printer.annotate(self.line_highlight.clone()))
    }
}

struct MultiLineFormatDisplay<'s> {
    line_numbers: RangeInclusive<usize>,
    must_be_visible_line_numbers: RangeInclusive<usize>,
    lines: &'s str,
    first_line_highlight: usize,
    last_line_highlight: usize,
}

impl Display for MultiLineFormatDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let gutter = Gutter::new(*self.line_numbers.end());
        let mut printer = LinePrinter::new(gutter, *self.line_numbers.start(), 1);
        let mut lines_iter = self.lines.split('\n');

        // Print the first line with the opening highlight.
        writeln!(f, "{}", gutter.empty())?;
        writeln!(f, "{}", printer.line(lines_iter.next().unwrap()))?;
        writeln!(
            f,
            "{}",
            printer.open(self.first_line_highlight..self.first_line_highlight)
        )?;

        // We might print up to three ranges of lines. The first range is the first two lines in
        // the highlight. The second range is the `must_be_visible_line_numbers`, with an extra line
        // before and after. The third range is the last two lines in the highlight. If any of
        // these ranges overlap or have a single line as gap, they are merged.
        let third_range = (*self.line_numbers.end() - 1)..=*self.line_numbers.end();
        let (second_range, third_range) =
            if *self.must_be_visible_line_numbers.end() + 2 < *third_range.start() {
                (
                    (*self.must_be_visible_line_numbers.start()
                        ..=*self.must_be_visible_line_numbers.end()),
                    Some(third_range),
                )
            } else {
                (
                    *self.must_be_visible_line_numbers.start()..=*third_range.end(),
                    None,
                )
            };
        let (first_range, second_range) = if *self.line_numbers.start() + 3 < *second_range.start()
        {
            (
                (*self.line_numbers.start() + 1)..=(*self.line_numbers.start() + 1),
                Some(second_range),
            )
        } else {
            ((*self.line_numbers.start() + 1)..=*second_range.end(), None)
        };

        let mut current_line_number = *self.line_numbers.start();
        for line in lines_iter
            .by_ref()
            .take(*first_range.end() - current_line_number)
        {
            writeln!(f, "{}", printer.line(line))?;
        }
        current_line_number = *first_range.end();

        for range in [second_range, third_range].into_iter().flatten() {
            let skip_count = *range.start() - current_line_number - 1;
            writeln!(f, "{}", printer.skip_lines(skip_count))?;

            current_line_number += skip_count;
            for line in lines_iter
                .by_ref()
                .skip(skip_count)
                .take(*range.end() - current_line_number)
            {
                writeln!(f, "{}", printer.line(line))?;
            }
            current_line_number = *range.end();
        }

        // Print the closing highlight.
        write!(
            f,
            "{}",
            printer.close(self.last_line_highlight..self.last_line_highlight)
        )
    }
}

fn get_line_containing(index: usize, val: &str) -> (usize, usize) {
    let line = val[..index].chars().filter(|ch| *ch == '\n').count() + 1;
    let line_start_index = val[..index].rfind('\n').map(|idx| idx + 1).unwrap_or(0);
    (line, line_start_index)
}
