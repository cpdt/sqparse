use crate::error_display::formats::{multi_line_format, single_line_format};
use std::fmt::{Display, Formatter};
use std::ops::Range;

mod formats;
mod gutter;
mod line_printer;
mod repeat;

pub fn display_error(
    text: &str,
    highlight: Range<usize>,
    visible: Range<usize>,
) -> impl Display + '_ {
    ErrorDisplay {
        text,
        highlight,
        visible,
    }
}

struct ErrorDisplay<'s> {
    text: &'s str,
    highlight: Range<usize>,
    visible: Range<usize>,
}

impl Display for ErrorDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let has_newline = self.text[self.highlight.clone()].contains('\n');

        if has_newline {
            write!(
                f,
                "{}",
                multi_line_format(self.text, self.highlight.clone(), self.visible.clone())
            )
        } else {
            write!(
                f,
                "{}",
                single_line_format(self.text, self.highlight.clone())
            )
        }
    }
}
