use crate::combinators::Formatter;
use crate::combinators::tuple_format::Tuple;

impl<I, O, F> Formatter<I, O> for F where F: FnOnce(I) -> Option<O> {
    fn format(self, input: I) -> Option<O> {
        self(input)
    }
}

pub fn alt<F1, F2, I, O>(f1: F1, f2: F2) -> impl FnOnce(I) -> Option<O>
    where F1: Formatter<I, O>,
          F2: Formatter<I, O>,
          I: Clone {
    move |i| {
        f1.format(i.clone()).or_else(|| f2.format(i))
    }
}

pub fn pair<F1, F2, I1, I2, O>(f1: F1, f2: F2) -> impl FnOnce(I1) -> Option<O>
    where F1: Formatter<I1, I2>,
          F2: Formatter<I2, O> {
    move |i| f2.format(f1.format(i)?)
}

pub fn tuple<I, T: Tuple<I>>(t: T) -> impl FnOnce(I) -> Option<I> {
    move |i| t.format(i)
}

pub fn iter<I, IntoIter, F>(iter: IntoIter) -> impl FnOnce(I) -> Option<I>
    where IntoIter: IntoIterator<Item=F>,
          F: Formatter<I, I> {
    move |mut i| {
        for f in iter {
            i = f.format(i)?;
        }
        Some(i)
    }
}

pub fn cond<F, I>(cond: bool, f: F) -> impl FnOnce(I) -> Option<I>
    where F: Formatter<I, I> {
    move |i| {
        if cond {
            f.format(i)
        } else {
            Some(i)
        }
    }
}

pub fn cond_or<F1, F2, I>(cond: bool, t: F1, f: F2) -> impl FnOnce(I) -> Option<I>
    where F1: Formatter<I, I>,
          F2: Formatter<I, I> {
    move |i| {
        if cond {
            t.format(i)
        } else {
            f.format(i)
        }
    }
}

pub fn opt<V, F1, F2, I>(val: Option<V>, f: F1) -> impl FnOnce(I) -> Option<I>
    where F1: FnOnce(V) -> F2,
          F2: Formatter<I, I> {
    move |i| {
        match val {
            Some(val) => f(val).format(i),
            None => Some(i),
        }
    }
}

pub fn opt_or<V, F1, F2, F3, I>(val: Option<V>, some: F1, none: F3) -> impl FnOnce(I) -> Option<I>
    where F1: FnOnce(V) -> F2,
          F2: Formatter<I, I>,
          F3: Formatter<I, I> {
    move |i| {
        match val {
            Some(val) => some(val).format(i),
            None => none.format(i),
        }
    }
}
