use std::fmt::{Display, Formatter};

pub fn repeat<T: Display>(times: usize, item: T) -> impl Display {
    RepeatDisplay(times, item)
}

struct RepeatDisplay<T: Display>(usize, T);

impl<T: Display> Display for RepeatDisplay<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.0 {
            self.1.fmt(f)?;
        }
        Ok(())
    }
}
