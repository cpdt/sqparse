//! A utility for pretty-printing source code annotations, warnings and errors.
//!
//! [`display_annotations`] returns an object that implements [`Display`], that will pretty-print
//! some source code based on a list of [`Annotation`]s.
//!
//! # Example
//! ```
//! use sqparse::annotation::{Annotation, display_annotations, Mode};
//!
//! yansi::Paint::disable();
//!
//! let source = "highlight me!";
//! let annotations = [
//!     Annotation {
//!         mode: Mode::Info,
//!         text: "this is me!".to_string(),
//!         note: "".to_string(),
//!         highlight: 10..12,
//!         visible: 10..12,
//!     }
//! ];
//! let annotations = format!("{}", display_annotations(Some("file.txt"), source, &annotations));
//! assert_eq!(annotations, " --> file.txt:1:11
//!   |
//! 1 | highlight me!
//!   |           -- this is me!");
//! ```

mod formats;
mod gutter;
mod line_printer;
mod mode;
mod repeat;

use crate::annotation::formats::{MultiLineFormatDisplay, SingleLineFormatDisplay};
use crate::annotation::gutter::Gutter;
use std::fmt::{Display, Formatter};
use std::ops::{Range, RangeInclusive};
use yansi::Paint;

pub use self::mode::Mode;

#[derive(Debug, Clone)]
pub struct Annotation {
    pub mode: Mode,
    pub text: String,
    pub note: String,
    pub highlight: Range<usize>,
    pub visible: Range<usize>,
}

pub fn display_annotations<'s>(
    file_name: Option<&'s str>,
    source: &'s str,
    annotations: &'s [Annotation],
) -> impl Display + 's {
    let format_data: Vec<_> = annotations
        .iter()
        .map(|annotation| {
            FormatData::new(
                source,
                annotation.highlight.clone(),
                annotation.visible.clone(),
            )
        })
        .collect();
    let max_line_number = format_data
        .iter()
        .map(|format| *format.line_numbers().end())
        .max()
        .unwrap_or(0);
    let gutter = Gutter::new(max_line_number);

    AnnotationsDisplay {
        file_name,
        annotations,
        gutter,
        format_data,
    }
}

struct AnnotationsDisplay<'s> {
    file_name: Option<&'s str>,
    annotations: &'s [Annotation],
    gutter: Gutter,
    format_data: Vec<FormatData<'s>>,
}

impl Display for AnnotationsDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.gutter.file())?;
        if let Some(file_name) = self.file_name {
            write!(f, "{file_name}:")?;
        }
        if let Some(first_format_data) = self.format_data.first() {
            write!(
                f,
                "{}:{}",
                first_format_data.line_numbers().start(),
                first_format_data.first_line_highlight() + 1
            )?;
        }

        for (annotation_index, (annotation, format_data)) in self
            .annotations
            .iter()
            .zip(self.format_data.iter())
            .enumerate()
        {
            writeln!(f, "\n{}", self.gutter.empty())?;

            match format_data {
                FormatData::SingleLine {
                    line,
                    line_number,
                    line_highlight,
                } => {
                    write!(
                        f,
                        "{}",
                        SingleLineFormatDisplay {
                            mode: annotation.mode,
                            gutter: self.gutter,
                            line,
                            line_number: *line_number,
                            line_highlight: line_highlight.clone()
                        },
                    )?;
                }
                FormatData::MultiLine {
                    lines,
                    line_numbers,
                    must_be_visible_line_numbers,
                    first_line_highlight,
                    last_line_highlight,
                } => {
                    write!(
                        f,
                        "{}",
                        MultiLineFormatDisplay {
                            mode: annotation.mode,
                            gutter: self.gutter,
                            lines,
                            line_numbers: line_numbers.clone(),
                            must_be_visible_line_numbers: must_be_visible_line_numbers.clone(),
                            first_line_highlight: *first_line_highlight,
                            last_line_highlight: *last_line_highlight,
                        },
                    )?;
                }
            }

            write!(f, " {}", annotation.mode.display(&annotation.text))?;

            if !annotation.note.is_empty() {
                writeln!(f, "\n{}", self.gutter.empty())?;
                write!(
                    f,
                    "{} {}",
                    self.gutter.separator(),
                    Paint::white(&annotation.note).bold()
                )?;
            } else if annotation_index + 1 < self.annotations.len() {
                write!(f, "\n{}", self.gutter.separator())?;
            }
        }

        Ok(())
    }
}

enum FormatData<'s> {
    SingleLine {
        line: &'s str,
        line_number: usize,
        line_highlight: Range<usize>,
    },
    MultiLine {
        lines: &'s str,
        line_numbers: RangeInclusive<usize>,
        must_be_visible_line_numbers: RangeInclusive<usize>,
        first_line_highlight: usize,
        last_line_highlight: usize,
    },
}

impl<'s> FormatData<'s> {
    pub fn new(text: &'s str, highlight: Range<usize>, visible: Range<usize>) -> Self {
        let has_newline = text[highlight.clone()].contains('\n');

        if has_newline {
            Self::new_multi_line(text, highlight, visible)
        } else {
            Self::new_single_line(text, highlight)
        }
    }

    pub fn line_numbers(&self) -> RangeInclusive<usize> {
        match self {
            FormatData::SingleLine { line_number, .. } => (*line_number)..=(*line_number),
            FormatData::MultiLine { line_numbers, .. } => line_numbers.clone(),
        }
    }

    pub fn first_line_highlight(&self) -> usize {
        match self {
            FormatData::SingleLine { line_highlight, .. } => line_highlight.start,
            FormatData::MultiLine {
                first_line_highlight,
                ..
            } => *first_line_highlight,
        }
    }

    fn new_single_line(text: &'s str, highlight: Range<usize>) -> Self {
        let (line_number, line_start_index) = get_line_containing(highlight.start, text);
        let text_from_start = &text[line_start_index..];

        let end_index = text_from_start.find('\n').unwrap_or(text_from_start.len());
        let line = &text_from_start[..end_index];

        let line_highlight =
            (highlight.start - line_start_index)..(highlight.end - line_start_index);

        FormatData::SingleLine {
            line,
            line_number,
            line_highlight,
        }
    }

    fn new_multi_line(text: &'s str, highlight: Range<usize>, visible: Range<usize>) -> Self {
        let (first_line_number, first_start_index) = get_line_containing(highlight.start, text);
        let (last_line_number, last_start_index) =
            get_line_containing(highlight.end.saturating_sub(1), text);

        let (first_must_be_visible_line_number, _) = get_line_containing(visible.start, text);
        let (last_must_be_visible_line_number, _) =
            get_line_containing(visible.end.saturating_sub(1), text);

        let last_end_index = text[last_start_index..]
            .find('\n')
            .map(|idx| last_start_index + idx)
            .unwrap_or(text.len());

        let lines = &text[first_start_index..last_end_index];
        let first_line_highlight = highlight.start - first_start_index;
        let last_line_highlight = highlight.end.saturating_sub(1) - last_start_index;

        FormatData::MultiLine {
            line_numbers: first_line_number..=last_line_number,
            must_be_visible_line_numbers: first_must_be_visible_line_number
                ..=last_must_be_visible_line_number,
            lines,
            first_line_highlight,
            last_line_highlight,
        }
    }
}

fn get_line_containing(index: usize, val: &str) -> (usize, usize) {
    let line = val[..index].chars().filter(|ch| *ch == '\n').count() + 1;
    let line_start_index = val[..index].rfind('\n').map(|idx| idx + 1).unwrap_or(0);
    (line, line_start_index)
}
