use crate::annotation::repeat::repeat;
use owo_colors::OwoColorize;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy)]
pub struct Gutter {
    number_width: usize,
}

impl Gutter {
    pub fn new(max_line_number: usize) -> Self {
        Gutter {
            number_width: max_line_number.to_string().len(),
        }
    }

    pub fn file(self) -> impl Display {
        GutterDisplay(FileGutterDisplay(self))
    }

    pub fn empty(self) -> impl Display {
        GutterDisplay(EmptyGutterDisplay(self))
    }

    pub fn separator(self) -> impl Display {
        GutterDisplay(SeparatorGutterDisplay(self))
    }

    pub fn ellipsis(self) -> impl Display {
        GutterDisplay(EllipsisGutterDisplay(self))
    }

    pub fn number(self, number: usize) -> impl Display {
        GutterDisplay(NumberGutterDisplay(self, number))
    }
}

struct GutterDisplay<T: Display>(T);

impl<T: Display> Display for GutterDisplay<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.bright_cyan())
    }
}

struct FileGutterDisplay(Gutter);

impl Display for FileGutterDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{pad}-->", pad = repeat(self.0.number_width, ' '))
    }
}

struct EmptyGutterDisplay(Gutter);

impl Display for EmptyGutterDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{pad} |", pad = repeat(self.0.number_width, ' '))
    }
}

struct SeparatorGutterDisplay(Gutter);

impl Display for SeparatorGutterDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{pad} =", pad = repeat(self.0.number_width, ' '))
    }
}

struct EllipsisGutterDisplay(Gutter);

impl Display for EllipsisGutterDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{pad}  ", pad = repeat(self.0.number_width, '.'))
    }
}

struct NumberGutterDisplay(Gutter, usize);

impl Display for NumberGutterDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{line_no: >max_width$} |",
            line_no = self.1,
            max_width = self.0.number_width
        )
    }
}
