pub trait Formatter<I, O> {
    fn format(self, input: I) -> Option<O>;
}
