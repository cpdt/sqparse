use std::fmt::{Display, Formatter};
use yansi::Paint;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Warning,
    Error,
    Info,
}

impl Mode {
    pub fn display<T: Display>(self, item: T) -> impl Display {
        ModeDisplay(self, item)
    }

    pub fn underline(self) -> char {
        match self {
            Mode::Warning | Mode::Error => '^',
            Mode::Info => '-',
        }
    }
}

struct ModeDisplay<T: Display>(Mode, T);

impl<T: Display> Display for ModeDisplay<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Mode::Warning => write!(f, "{}", Paint::yellow(&self.1).bold()),
            Mode::Error => write!(f, "{}", Paint::red(&self.1).bold()),
            Mode::Info => write!(f, "{}", Paint::cyan(&self.1).bold()),
        }
    }
}
