use crate::annotation::gutter::Gutter;
use crate::annotation::line_printer::LinePrinter;
use crate::annotation::Mode;
use std::fmt::{Display, Formatter};
use std::ops::{Range, RangeInclusive};

pub struct SingleLineFormatDisplay<'s> {
    pub mode: Mode,
    pub gutter: Gutter,
    pub line: &'s str,
    pub line_number: usize,
    pub line_highlight: Range<usize>,
}

impl Display for SingleLineFormatDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut printer = LinePrinter::new(self.mode, self.gutter, self.line_number, 0);

        if self.line.len() > 120 {
            // Print 120 characters centered around the center of the highlight range.
            let highlight_center = (self.line_highlight.start + self.line_highlight.end) / 2;
            let display_start = highlight_center
                .saturating_sub(120 / 2)
                .min(self.line.len().saturating_sub(120));
            let display_end = display_start + 120;

            let is_start_elided = display_start > 4;
            let is_end_elided = display_end < self.line.len() - 4;

            let elided_start = if is_start_elided {
                display_start + 4
            } else {
                display_start
            };
            let elided_end = if is_end_elided {
                display_end - 4
            } else {
                display_end
            };

            write!(f, "{}", printer.line(""))?;

            if is_start_elided {
                write!(f, "... ")?;
            }
            write!(f, "{}", &self.line[elided_start..elided_end])?;
            if is_end_elided {
                write!(f, " ...")?;
            }

            writeln!(f)?;
            write!(
                f,
                "{}",
                printer.annotate(
                    (self.line_highlight.start.max(display_start) - display_start)
                        ..(self.line_highlight.end.min(display_end) - display_start)
                )
            )?;
        } else {
            writeln!(f, "{}", printer.line(self.line))?;
            write!(f, "{}", printer.annotate(self.line_highlight.clone()))?;
        }

        Ok(())
    }
}

pub struct MultiLineFormatDisplay<'s> {
    pub mode: Mode,
    pub gutter: Gutter,
    pub lines: &'s str,
    pub line_numbers: RangeInclusive<usize>,
    pub must_be_visible_line_numbers: RangeInclusive<usize>,
    pub first_line_highlight: usize,
    pub last_line_highlight: usize,
}

impl Display for MultiLineFormatDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut printer = LinePrinter::new(self.mode, self.gutter, *self.line_numbers.start(), 1);

        let mut lines_iter = self.lines.split('\n');

        // Print the first line with the opening highlight.
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
