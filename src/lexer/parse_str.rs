#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseStr<'s> {
    val: &'s str,
    offset: usize,
}

impl<'s> ParseStr<'s> {
    pub fn new(val: &'s str) -> ParseStr<'s> {
        ParseStr { val, offset: 0 }
    }

    pub fn as_str(self) -> &'s str {
        self.val
    }

    pub fn from(self, idx: usize) -> ParseStr<'s> {
        ParseStr {
            val: &self.val[idx..],
            offset: self.offset + idx,
        }
    }

    pub fn end(self) -> ParseStr<'static> {
        ParseStr {
            val: "",
            offset: self.offset + self.val.len(),
        }
    }

    pub fn len(self) -> usize {
        self.val.len()
    }

    pub fn is_ended(self) -> bool {
        self.val.is_empty()
    }

    pub fn start_offset(self) -> usize {
        self.offset
    }

    pub fn end_offset(self) -> usize {
        self.offset + self.val.len()
    }

    pub fn split_at(self, mid: impl Into<Option<usize>>) -> (&'s str, ParseStr<'s>) {
        match mid.into() {
            Some(mid) => (&self.val[..mid], self.from(mid)),
            None => (self.val, self.end()),
        }
    }

    pub fn trim_start(self) -> ParseStr<'s> {
        match self.val.find(|c: char| c == '\n' || !c.is_whitespace()) {
            Some(start_index) => self.from(start_index),
            None => self.end(),
        }
    }

    pub fn strip_prefix(self, prefix: &str) -> Option<ParseStr<'s>> {
        self.val.strip_prefix(prefix).map(|val| ParseStr {
            val,
            offset: self.offset + prefix.len(),
        })
    }
}
