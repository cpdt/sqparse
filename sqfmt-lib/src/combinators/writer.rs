use crate::config::Format;
use crate::writer::Writer;

pub fn definitely_multi_line<F: FnOnce(Writer) -> Option<Writer>>(f: F) -> impl FnOnce(Writer) -> Option<Writer> {
    move |i| {
        if i.is_single_line() {
            return None;
        }
        f(i)
    }
}

pub fn single_line<F: FnOnce(Writer) -> Option<Writer>>(f: F) -> impl FnOnce(Writer) -> Option<Writer> {
    move |i| i.with_single_line(f)
}

pub fn indented<F: FnOnce(Writer) -> Option<Writer>>(f: F) -> impl FnOnce(Writer) -> Option<Writer> {
    move |i| i.with_indent(f)
}

pub fn empty_line(i: Writer) -> Option<Writer> {
    i.empty_line()
}

pub fn new_line(i: Writer) -> Option<Writer> {
    i.write_new_line()
}

pub fn space(i: Writer) -> Option<Writer> {
    Some(i.write_space())
}

pub fn format<C, F>(c: C, f: F) -> impl FnOnce(Writer) -> Option<Writer>
    where C: FnOnce(&Format) -> bool,
          F: FnOnce(Writer) -> Option<Writer> {
    move |i| {
        if c(i.format()) {
            f(i)
        } else {
            Some(i)
        }
    }
}

pub fn tag(val: &'static str) -> impl FnOnce(Writer) -> Option<Writer> {
    move |i| i.write(val)
}
