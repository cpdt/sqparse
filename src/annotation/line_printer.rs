use crate::annotation::gutter::Gutter;
use crate::annotation::mode::Mode;
use crate::annotation::repeat::repeat;
use std::fmt::{Display, Formatter};
use std::ops::Range;

pub struct LinePrinter {
    mode: Mode,
    gutter: Gutter,
    next_line_number: usize,
    bars: Bars,
}

impl LinePrinter {
    pub fn new(
        mode: Mode,
        gutter: Gutter,
        first_line_number: usize,
        max_depth: usize,
    ) -> LinePrinter {
        LinePrinter {
            mode,
            gutter,
            next_line_number: first_line_number,
            bars: Bars {
                mode,
                max_depth,
                current_depth: 0,
            },
        }
    }

    pub fn line<'s>(&mut self, line_text: &'s str) -> impl Display + 's {
        let display = LineDisplay {
            gutter: self.gutter,
            line_number: self.next_line_number,
            bars: self.bars,
            line_text,
        };
        self.next_line_number += 1;
        display
    }

    pub fn skip_lines(&mut self, lines: usize) -> impl Display {
        self.next_line_number += lines;
        SkipDisplay {
            gutter: self.gutter,
            bars: self.bars,
        }
    }

    pub fn annotate(&self, highlight: Range<usize>) -> impl Display {
        AnnotateDisplay {
            mode: self.mode,
            gutter: self.gutter,
            bars: self.bars,
            highlight,
        }
    }

    pub fn open(&mut self, highlight: Range<usize>) -> impl Display {
        let display = OpenCloseDisplay {
            mode: self.mode,
            gutter: self.gutter,
            bars: self.bars,
            highlight,
        };
        assert!(self.bars.current_depth < self.bars.max_depth);
        self.bars.current_depth += 1;
        display
    }

    pub fn close(&mut self, highlight: Range<usize>) -> impl Display {
        let display = OpenCloseDisplay {
            mode: self.mode,
            gutter: self.gutter,
            bars: self.bars,
            highlight,
        };
        assert!(self.bars.current_depth > 0);
        self.bars.current_depth -= 1;
        display
    }
}

struct LineDisplay<'s> {
    gutter: Gutter,
    line_number: usize,
    bars: Bars,
    line_text: &'s str,
}

impl Display for LineDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let no_tabs_line = self.line_text.replace('\t', " ");
        write!(
            f,
            "{gutter}{bars} {text}",
            gutter = self.gutter.number(self.line_number),
            bars = self.bars,
            text = no_tabs_line,
        )
    }
}

struct SkipDisplay {
    gutter: Gutter,
    bars: Bars,
}

impl Display for SkipDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{gutter}{bars}",
            gutter = self.gutter.ellipsis(),
            bars = self.bars,
        )
    }
}

struct AnnotateDisplay {
    mode: Mode,
    gutter: Gutter,
    bars: Bars,
    highlight: Range<usize>,
}

impl Display for AnnotateDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{gutter}{bars} {offset}{underline}",
            gutter = self.gutter.empty(),
            bars = self.bars,
            offset = repeat(self.highlight.start, ' '),
            underline = self
                .mode
                .display(repeat(self.highlight.len().max(1), self.mode.underline())),
        )
    }
}

struct OpenCloseDisplay {
    mode: Mode,
    gutter: Gutter,
    bars: Bars,
    highlight: Range<usize>,
}

impl Display for OpenCloseDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{gutter}{bars}{offset}{underline}",
            gutter = self.gutter.empty(),
            bars = self.bars,
            offset = self.mode.display(repeat(self.highlight.start + 1, '_')),
            underline = self.mode.display(repeat(self.highlight.len().max(1), '^')),
        )
    }
}

#[derive(Clone, Copy)]
struct Bars {
    mode: Mode,
    max_depth: usize,
    current_depth: usize,
}

impl Display for Bars {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{bars}{padding}",
            bars = self.mode.display(repeat(self.current_depth, " |")),
            padding = repeat(self.max_depth - self.current_depth, "  "),
        )
    }
}
